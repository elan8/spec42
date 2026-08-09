# META
~~~ini
description=SysML Example (v1 Spec): Wheel Package - Updated
type=file
~~~
# SOURCE
~~~sysml
package 'Wheel Package - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

	private import ISQ::*;
	
	// Quantities
	
	pressure = force / length^2; 
	
	// Blocks
	
	part def WheelHubAssembly;
	part def WheelAssembly {
		inflationPressure :> pressure;
	}
	
	part def Tire {
		tireSpecification : ScalarValues::String;		
		action mountTire; // Should be operation
	}
	
	part def TireBead;
	
	connection def PressureSeat {
		end : TireBead[1];
		end : TireMountingRim[1];
	}
	
	part def Wheel {
		diameter :> length;
		width :> length;		
	}
	
	connection def BandMount {
		end : Wheel[1];
		end : WirelessTirePressureMonitor[1];
	}
	
	part def WirelessTirePressureMonitor {
		action transmitPressure; // Should be operation
	}
	
	part def TireMountingRim;
	
	part def InflationValve;
	
	part def BalanceWeight;
	
	part def LugBoltMountingHole {
		lugBoltSize :> length;
	}
	
	part def LugBoltJoint {
		torque :> ISQ::torque;
		boltTension :> force;
	}
	
	part def Hub;
	
	part def LugBoltThreadableHole {
		lugBoltSize :> length;
		threadSize :> length;
	}
	
	// Parts
	
	part wheelHubAssembly: WheelHubAssembly {
		part wheel: WheelAssembly[1] {
			part t: Tire[1] {
				part bead : TireBead[2];			
			}
			part w: Wheel[1] {
				part rim : TireMountingRim[2];
				part v : InflationValve[1];
				part weight : BalanceWeight[0..6];
				part mountingHoles : LugBoltMountingHole[5];
			}						
			connection : PressureSeat connect t.bead to w.rim;		
		}
		part lugBoltJoints: LugBoltJoint[5] {					
			ref mountingHole: LugBoltMountingHole[1] subsets wheel.w.mountingHoles;
			ref threadedHole: LugBoltThreadableHole[1] subsets hub.h;
		}
		part hub: Hub[1] {
			part h: LugBoltThreadableHole[5];
		}
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
LineComment,
Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
LineComment,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwAction,Ident,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAction,Ident,Semicolon,LineComment,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
LineComment,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,Colon,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Dot,Ident,Dot,Ident,Semicolon,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,KwSubsets,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Wheel Package - Updated''
    (documentation)
    (import_decl private 'ISQ::*')
    (line_comment)
    (feature_def 'pressure' value)
    (line_comment)
    (part_def 'WheelHubAssembly')
    (part_def 'WheelAssembly'
      (default_ref_usage 'inflationPressure' :> 'pressure'))
    (part_def 'Tire'
      (default_ref_usage 'tireSpecification' : 'ScalarValues::String')
      (action_usage 'mountTire')
      (line_comment))
    (part_def 'TireBead')
    (connection_def 'PressureSeat'
      (interface_end end : 'TireBead')
      (interface_end end : 'TireMountingRim'))
    (part_def 'Wheel'
      (default_ref_usage 'diameter' :> 'length')
      (default_ref_usage 'width' :> 'length'))
    (connection_def 'BandMount'
      (interface_end end : 'Wheel')
      (interface_end end : 'WirelessTirePressureMonitor'))
    (part_def 'WirelessTirePressureMonitor'
      (action_usage 'transmitPressure')
      (line_comment))
    (part_def 'TireMountingRim')
    (part_def 'InflationValve')
    (part_def 'BalanceWeight')
    (part_def 'LugBoltMountingHole'
      (default_ref_usage 'lugBoltSize' :> 'length'))
    (part_def 'LugBoltJoint'
      (default_ref_usage 'torque' :> 'ISQ::torque')
      (default_ref_usage 'boltTension' :> 'force'))
    (part_def 'Hub')
    (part_def 'LugBoltThreadableHole'
      (default_ref_usage 'lugBoltSize' :> 'length')
      (default_ref_usage 'threadSize' :> 'length'))
    (line_comment)
    (part_usage 'wheelHubAssembly' : 'WheelHubAssembly'
      (part_usage 'wheel' : 'WheelAssembly' multiplicity
        (part_usage 't' : 'Tire' multiplicity
          (part_usage 'bead' : 'TireBead' multiplicity))
        (part_usage 'w' : 'Wheel' multiplicity
          (part_usage 'rim' : 'TireMountingRim' multiplicity)
          (part_usage 'v' : 'InflationValve' multiplicity)
          (part_usage 'weight' : 'BalanceWeight' multiplicity)
          (part_usage 'mountingHoles' : 'LugBoltMountingHole' multiplicity))
        (connection_usage 'PressureSeat'
          (connector_end)
          (connector_end)))
      (part_usage 'lugBoltJoints' : 'LugBoltJoint' multiplicity
        (ref_usage ref 'mountingHole' : 'LugBoltMountingHole' :> 'wheel.w.mountingHoles' multiplicity)
        (ref_usage ref 'threadedHole' : 'LugBoltThreadableHole' :> 'hub.h' multiplicity))
      (part_usage 'hub' : 'Hub' multiplicity
        (part_usage 'h' : 'LugBoltThreadableHole' multiplicity)))))
~~~
# FORMAT
~~~sysml
package 'Wheel Package - Updated' {
    doc /*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

    private import ISQ::*;

    // Quantities

    pressure = force / length^2;

    // Blocks

    part def WheelHubAssembly;
    part def WheelAssembly {
        inflationPressure :> pressure;
    }

    part def Tire {
        tireSpecification : ScalarValues::String;
        action mountTire;
        // Should be operation
    }

    part def TireBead;

    connection def PressureSeat {
        end : TireBead;
        end : TireMountingRim;
    }

    part def Wheel {
        diameter :> length;
        width :> length;
    }

    connection def BandMount {
        end : Wheel;
        end : WirelessTirePressureMonitor;
    }

    part def WirelessTirePressureMonitor {
        action transmitPressure;
        // Should be operation
    }

    part def TireMountingRim;

    part def InflationValve;

    part def BalanceWeight;

    part def LugBoltMountingHole {
        lugBoltSize :> length;
    }

    part def LugBoltJoint {
        torque :> ISQ::torque;
        boltTension :> force;
    }

    part def Hub;

    part def LugBoltThreadableHole {
        lugBoltSize :> length;
        threadSize :> length;
    }

    // Parts

    part wheelHubAssembly : WheelHubAssembly {
        part wheel : WheelAssembly [1] {
            part t : Tire [1] {
                part bead : TireBead [2];
            }
            part w : Wheel [1] {
                part rim : TireMountingRim [2];
                part v : InflationValve [1];
                part weight : BalanceWeight [0..6];
                part mountingHoles : LugBoltMountingHole [5];
            }
            connection : PressureSeat connect t.bead to w.rim;
        }
        part lugBoltJoints : LugBoltJoint [5] {
            ref mountingHole : LugBoltMountingHole subsets wheel.w.mountingHoles [1];
            ref threadedHole : LugBoltThreadableHole subsets hub.h [1];
        }
        part hub : Hub [1] {
            part h : LugBoltThreadableHole [5];
        }
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'force'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ScalarValues::String'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
semantic.unresolved_name 'ISQ::torque'
semantic.unresolved_name 'force'
semantic.unresolved_name 'length'
semantic.unresolved_name 'length'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Wheel Package - Updated"))) (name "Wheel Package - Updated") (declared-name "Wheel Package - Updated")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Wheel Package - Updated::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (name "BalanceWeight") (declared-name "BalanceWeight") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::BandMount"))) (name "BandMount") (declared-name "BandMount"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::Hub"))) (name "Hub") (declared-name "Hub") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::InflationValve"))) (name "InflationValve") (declared-name "InflationValve") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (name "LugBoltJoint") (declared-name "LugBoltJoint") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::PressureSeat"))) (name "PressureSeat") (declared-name "PressureSeat"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::Tire"))) (name "Tire") (declared-name "Tire") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Wheel Package - Updated::Tire::mountTire"))) (name "mountTire") (declared-name "mountTire") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Tire")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::TireBead"))) (name "TireBead") (declared-name "TireBead") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (name "TireMountingRim") (declared-name "TireMountingRim") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (name "WheelAssembly") (declared-name "WheelAssembly") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor"))) (name "WirelessTirePressureMonitor") (declared-name "WirelessTirePressureMonitor") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor::transmitPressure"))) (name "transmitPressure") (declared-name "transmitPressure") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Wheel Package - Updated::_documentation"))) (name ""))
        (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (name "wheelHubAssembly") (declared-name "wheelHubAssembly") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (name "hub") (declared-name "hub") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (name "h") (declared-name "h") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Hub")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (name "lugBoltJoints") (declared-name "lugBoltJoints") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (name "t") (declared-name "t") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (name "bead") (declared-name "bead") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Tire")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (name "w") (declared-name "w") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (name "mountingHoles") (declared-name "mountingHoles") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (name "rim") (declared-name "rim") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (name "v") (declared-name "v") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (name "weight") (declared-name "weight") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 0) (upper 6) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::_documentation"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::Hub"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltJoint"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::Tire"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::TireBead"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::TireMountingRim"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::InflationValve"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (to (node (document "d0") (qualified-name "Wheel Package - Updated::BalanceWeight"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
