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
    doc /*
	 * Example vehicle definitions model.
	 */

    /* BLOCKS */

    class Vehicle;
    class Transmission;
    class AxleAssembly;
    class Axle;
    class Wheel;
    class Lugbolt {
        tighteningTorque[1]: ScalarValues::Real;
    }

    /* INTERFACE BLOCKS */

    class DriveIF {
        in driveTorque: ScalarValues::Real;
    }

    class AxleMountIF {
        out transferredTorque: ScalarValues::Real;
    }

    class WheelHubIF {
        in appliedTorque: ScalarValues::Real;
    }

    /* ASSOCIATION BLOCKS */

    assoc Mounting {
        doc /*
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
(model
  (namespace
    (package 'VehicleDefinitions'
      (documentation)
      (class_def 'Vehicle')
      (class_def 'Transmission')
      (class_def 'AxleAssembly')
      (class_def 'Axle')
      (class_def 'Wheel')
      (class_def 'Lugbolt'
        (feature_def 'tighteningTorque' : 'ScalarValues::Real'[unresolved]
          (multiplicity_range [1])))
      (class_def 'DriveIF'
        (feature_def in 'driveTorque' : 'ScalarValues::Real'[unresolved]))
      (class_def 'AxleMountIF'
        (feature_def out 'transferredTorque' : 'ScalarValues::Real'[unresolved]))
      (class_def 'WheelHubIF'
        (feature_def in 'appliedTorque' : 'ScalarValues::Real'[unresolved]))
      (association_def 'Mounting'
        (documentation)
        (feature_def end 'axleMount' : 'VehicleDefinitions::AxleMountIF'[class_def])
        (feature_def end 'hub' : 'VehicleDefinitions::WheelHubIF'[class_def])))))
~~~
