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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
RegularComment,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPort,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
RegularComment,
KwPort,KwDef,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwOut,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPort,KwDef,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
RegularComment,
KwInterface,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwEnd,Ident,Colon,Ident,Semicolon,
KwEnd,Ident,Colon,Ident,Semicolon,
KwFlow,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'VehicleDefinitions'
    (documentation)
    (import_decl private 'ScalarValues::*')
    (import_decl private 'Quantities::*')
    (import_decl private 'MeasurementReferences::*')
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (comment)
    (part_def 'Vehicle'
      (attribute_usage 'mass' :> 'ISQ::mass'))
    (part_def 'Transmission')
    (part_def 'AxleAssembly')
    (part_def 'Axle'
      (port_usage 'leftMountingPoint' : 'AxleMountIF')
      (port_usage 'rightMountingPoint' : 'AxleMountIF'))
    (part_def 'Wheel'
      (port_usage 'hub' : 'WheelHubIF'))
    (part_def 'Lugbolt'
      (attribute_usage 'tighteningTorque' :> 'ISQ::torque'))
    (comment)
    (port_def 'DriveIF'
      (default_ref_usage in 'driveTorque' :> 'ISQ::torque'))
    (port_def 'AxleMountIF'
      (default_ref_usage out 'transferredTorque' :> 'ISQ::torque'))
    (port_def 'WheelHubIF'
      (default_ref_usage in 'appliedTorque' :> 'ISQ::torque'))
    (comment)
    (interface_def 'Mounting'
      (documentation)
      (interface_end end 'axleMount' : 'AxleMountIF')
      (interface_end end 'hub' : 'WheelHubIF')
      (flow_usage 'axleMount'))))
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
# EXPECTED
~~~
semantic.duplicate_name 'axleMount'
semantic.invalid_connection_end_count
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::torque'
~~~
# PROBLEMS
~~~
semantic.duplicate_name 'axleMount'
semantic.invalid_connection_end_count
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'ISQ::torque'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "VehicleDefinitions"))) (name "VehicleDefinitions") (declared-name "VehicleDefinitions")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleDefinitions::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import3"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "VehicleDefinitions::*#import4"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (name "Axle") (declared-name "Axle") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (name "leftMountingPoint") (declared-name "leftMountingPoint") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Axle")))))
            (element (kind "port") (id (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (name "rightMountingPoint") (declared-name "rightMountingPoint") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Axle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (name "AxleAssembly") (declared-name "AxleAssembly") (declared))
        (element (kind "port def") (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (name "AxleMountIF") (declared-name "AxleMountIF")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (name "transferredTorque") (declared-name "transferredTorque") (declared (properties (direction "out"))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::~AxleMountIF"))) (name "~AxleMountIF") (declared-name "~AxleMountIF") (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (name "DriveIF") (declared-name "DriveIF")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::driveTorque"))) (name "driveTorque") (declared-name "driveTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::~DriveIF"))) (name "~DriveIF") (declared-name "~DriveIF") (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (name "Lugbolt") (declared-name "Lugbolt") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (name "tighteningTorque") (declared-name "tighteningTorque") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt")))))
          )
        )
        (element (kind "interface def") (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (name "Mounting") (declared-name "Mounting")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Mounting")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (name "axleMount") (declared-name "axleMount") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Mounting")))))
            (element (kind "interface end") (id (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (name "hub") (declared-name "hub") (declared (properties (end true))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Mounting")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (name "Transmission") (declared-name "Transmission") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
          (contains
            (element (kind "port") (id (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (name "hub") (declared-name "hub") (declared) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::Wheel")))))
          )
        )
        (element (kind "port def") (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (name "WheelHubIF") (declared-name "WheelHubIF")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (name "appliedTorque") (declared-name "appliedTorque") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF")))))
            (element (kind "conjugated port definition") (id (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::~WheelHubIF"))) (name "~WheelHubIF") (declared-name "~WheelHubIF") (effective (featuring-type (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::_documentation"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (provenance authored))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::_documentation"))) (to (node (document "d0") (qualified-name "VehicleDefinitions"))) (provenance authored))
    (connection (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored))
    (flow (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::transferredTorque"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::appliedTorque"))) (flow (source-expression "axleMount::transferredTorque") (target-expression "hub::appliedTorque")) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::~AxleMountIF"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::~DriveIF"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (provenance authored))
    (portConjugation (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::~WheelHubIF"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::axleMount"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::Mounting::hub"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (to (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Axle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Axle::leftMountingPoint"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Axle::rightMountingPoint"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::AxleAssembly"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::AxleMountIF::~AxleMountIF"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::DriveIF::~DriveIF"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Lugbolt::tighteningTorque"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Mounting"))) (status missing-prerequisite) (target "Interfaces::Interface"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Transmission"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Wheel"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::Wheel::hub"))) (status missing-prerequisite) (target "Ports::ports"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF"))) (status missing-prerequisite) (target "Ports::Port"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "VehicleDefinitions::WheelHubIF::~WheelHubIF"))) (status missing-prerequisite) (target "Ports::Port"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/vehicle_definitions.md"
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
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 33 2) (end 33 32))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 37 2) (end 37 39))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 41 2) (end 41 34))
      )
    )
  )
)
~~~
