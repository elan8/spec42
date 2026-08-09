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
    doc /*
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
        port leftMountingPoint : AxleMountIF;
        port rightMountingPoint : AxleMountIF;
    }
    part def Wheel {
        port hub : WheelHubIF;
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
        end axleMount : AxleMountIF;
        end hub : WheelHubIF;

        flow axleMount;
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
(model
  (namespace
    (package 'VehicleDefinitions'
      (documentation)
      (namespace_import private -> 'ScalarValues'[unresolved])
      (namespace_import private -> 'Quantities'[unresolved])
      (namespace_import private -> 'MeasurementReferences'[unresolved])
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (part_def 'Vehicle'
        (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
      (part_def 'Transmission')
      (part_def 'AxleAssembly')
      (part_def 'Axle'
        (port_usage composite 'leftMountingPoint' : 'VehicleDefinitions::AxleMountIF'[port_def])
        (port_usage composite 'rightMountingPoint' : 'VehicleDefinitions::AxleMountIF'[port_def]))
      (part_def 'Wheel'
        (port_usage composite 'hub' : 'VehicleDefinitions::WheelHubIF'[port_def]))
      (part_def 'Lugbolt'
        (attribute_usage composite 'tighteningTorque' :> 'ISQ::torque'[unresolved]))
      (port_def 'DriveIF'
        (reference_usage in reference 'driveTorque' :> 'ISQ::torque'[unresolved]))
      (port_def 'AxleMountIF'
        (reference_usage out reference 'transferredTorque' :> 'ISQ::torque'[unresolved]))
      (port_def 'WheelHubIF'
        (reference_usage in reference 'appliedTorque' :> 'ISQ::torque'[unresolved]))
      (interface_def 'Mounting'
        (documentation)
        (port_usage end 'axleMount' : 'VehicleDefinitions::AxleMountIF'[port_def])
        (port_usage end 'hub' : 'VehicleDefinitions::WheelHubIF'[port_def])
        (flow_usage composite 'axleMount')))))
~~~
