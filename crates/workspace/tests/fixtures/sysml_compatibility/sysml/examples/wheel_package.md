# META
~~~ini
description=SysML Example (v1 Spec): Wheel Package
type=file
~~~
# SOURCE
~~~sysml
package 'Wheel Package' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

	private import ISQ::*;
	
	pressure = force / length^2; 
	
	part def WheelHubAssembly {
		part wheel: WheelAssembly[1];
		part lugBoltJoints: LugBoltJoint[5] {
			ref redefines threadedHole subsets hub.h;
			ref redefines mountingHole subsets wheel.w.mountingHoles;
		}
		part hub: Hub[1];
	}
	
	part def WheelAssembly {
		inflationPressure :> pressure;
		
		part t: Tire[1] {
			part bead redefines Tire::bead;
		}
		part w: Wheel[1] {
			part rim redefines Wheel::rim;
		}		
				
		connection : PressureSeat connect t.bead to w.rim;		
	}
	
	part def Tire {
		tireSpecification : ScalarValues::String;
		
		part bead : TireBead[2];
		
		action mountTire;
	}
	
	part def TireBead;
	
	connection def PressureSeat {
		end : TireBead[1];
		end : TireMountingRim[1];
	}
	
	part def Wheel {
		diameter :> length;
		width :> length;
		
		part rim : TireMountingRim[2];
		part v : InflationValve[1];
		part weight : BalanceWeight[0..6];
		part mountingHoles : LugBoltMountingHole[5];
	}
	
	connection def BandMount {
		end : Wheel[1];
		end : WirelessTirePressureMonitor[1];
	}
	
	part def WirelessTirePressureMonitor {
		action transmitPressure;
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
		
		ref mountingHole: LugBoltMountingHole[1];
		ref threadedHole: LugBoltThreadableHole[1];
	}
	
	part def Hub {
		part h: LugBoltThreadableHole[5];
	}
	
	part def LugBoltThreadableHole {
		lugBoltSize :> length;
		threadSize :> length;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwDoc,
RegularComment,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
Ident,Eq,Ident,Slash,Ident,Caret,DecimalValue,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwRef,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Semicolon,
KwRef,KwRedefines,Ident,KwSubsets,Ident,Dot,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,OpenCurly,
KwPart,Ident,KwRedefines,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwConnection,Colon,Ident,KwConnect,Ident,Dot,Ident,KwTo,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAction,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,DotDot,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwConnection,KwDef,Ident,OpenCurly,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwEnd,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAction,Ident,Semicolon,
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
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwRef,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,ColonGt,Ident,Semicolon,
Ident,ColonGt,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Wheel Package''
    (documentation)
    (import_decl private 'ISQ::*')
    (feature_def 'pressure' value)
    (part_def 'WheelHubAssembly'
      (part_usage 'wheel' : 'WheelAssembly' multiplicity)
      (part_usage 'lugBoltJoints' : 'LugBoltJoint' multiplicity
        (ref_usage ref :>> 'threadedHole' :> 'hub.h')
        (ref_usage ref :>> 'mountingHole' :> 'wheel.w.mountingHoles'))
      (part_usage 'hub' : 'Hub' multiplicity))
    (part_def 'WheelAssembly'
      (default_ref_usage 'inflationPressure' :> 'pressure')
      (part_usage 't' : 'Tire' multiplicity
        (part_usage 'bead' :>> 'Tire::bead'))
      (part_usage 'w' : 'Wheel' multiplicity
        (part_usage 'rim' :>> 'Wheel::rim'))
      (connection_usage 'PressureSeat'
        (connector_end)
        (connector_end)))
    (part_def 'Tire'
      (default_ref_usage 'tireSpecification' : 'ScalarValues::String')
      (part_usage 'bead' : 'TireBead' multiplicity)
      (action_usage 'mountTire'))
    (part_def 'TireBead')
    (connection_def 'PressureSeat'
      (interface_end end : 'TireBead')
      (interface_end end : 'TireMountingRim'))
    (part_def 'Wheel'
      (default_ref_usage 'diameter' :> 'length')
      (default_ref_usage 'width' :> 'length')
      (part_usage 'rim' : 'TireMountingRim' multiplicity)
      (part_usage 'v' : 'InflationValve' multiplicity)
      (part_usage 'weight' : 'BalanceWeight' multiplicity)
      (part_usage 'mountingHoles' : 'LugBoltMountingHole' multiplicity))
    (connection_def 'BandMount'
      (interface_end end : 'Wheel')
      (interface_end end : 'WirelessTirePressureMonitor'))
    (part_def 'WirelessTirePressureMonitor'
      (action_usage 'transmitPressure'))
    (part_def 'TireMountingRim')
    (part_def 'InflationValve')
    (part_def 'BalanceWeight')
    (part_def 'LugBoltMountingHole'
      (default_ref_usage 'lugBoltSize' :> 'length'))
    (part_def 'LugBoltJoint'
      (default_ref_usage 'torque' :> 'ISQ::torque')
      (default_ref_usage 'boltTension' :> 'force')
      (ref_usage ref 'mountingHole' : 'LugBoltMountingHole' multiplicity)
      (ref_usage ref 'threadedHole' : 'LugBoltThreadableHole' multiplicity))
    (part_def 'Hub'
      (part_usage 'h' : 'LugBoltThreadableHole' multiplicity))
    (part_def 'LugBoltThreadableHole'
      (default_ref_usage 'lugBoltSize' :> 'length')
      (default_ref_usage 'threadSize' :> 'length'))))
~~~
# FORMAT
~~~sysml
package 'Wheel Package' {
    doc
    /*
	 * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.
	 */

    private import ISQ::*;

    pressure = force / length^2;

    part def WheelHubAssembly {
        part wheel: WheelAssembly[1];
        part lugBoltJoints: LugBoltJoint[5] {
            ref redefines threadedHole subsets hub.h;
            ref redefines mountingHole subsets wheel.w.mountingHoles;
        }
        part hub: Hub[1];
    }

    part def WheelAssembly {
        inflationPressure :> pressure;

        part t: Tire[1] {
            part bead redefines Tire::bead;
        }
        part w: Wheel[1] {
            part rim redefines Wheel::rim;
        }

        connection : PressureSeat connect t.bead to w.rim;
    }

    part def Tire {
        tireSpecification : ScalarValues::String;

        part bead : TireBead[2];

        action mountTire;
    }

    part def TireBead;

    connection def PressureSeat {
        end : TireBead[1];
        end : TireMountingRim[1];
    }

    part def Wheel {
        diameter :> length;
        width :> length;

        part rim : TireMountingRim[2];
        part v : InflationValve[1];
        part weight : BalanceWeight[0..6];
        part mountingHoles : LugBoltMountingHole[5];
    }

    connection def BandMount {
        end : Wheel[1];
        end : WirelessTirePressureMonitor[1];
    }

    part def WirelessTirePressureMonitor {
        action transmitPressure;
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

        ref mountingHole: LugBoltMountingHole[1];
        ref threadedHole: LugBoltThreadableHole[1];
    }

    part def Hub {
        part h: LugBoltThreadableHole[5];
    }

    part def LugBoltThreadableHole {
        lugBoltSize :> length;
        threadSize :> length;
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
    (element (kind "package") (id (node (document "d0") (qualified-name "Wheel Package"))) (name "Wheel Package") (declared-name "Wheel Package")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Wheel Package::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::BalanceWeight"))) (name "BalanceWeight") (declared-name "BalanceWeight") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Wheel Package::BandMount"))) (name "BandMount") (declared-name "BandMount"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::Hub"))) (name "Hub") (declared-name "Hub") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (name "h") (declared-name "h") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Hub")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::InflationValve"))) (name "InflationValve") (declared-name "InflationValve") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))) (name "LugBoltJoint") (declared-name "LugBoltJoint") (declared)
          (contains
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint::mountingHole"))) (name "mountingHole") (declared-name "mountingHole") (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint")))))
            (element (kind "opaque member") (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint::threadedHole"))) (name "threadedHole") (declared-name "threadedHole") (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole"))) (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole"))) (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (declared))
        (element (kind "connection def") (id (node (document "d0") (qualified-name "Wheel Package::PressureSeat"))) (name "PressureSeat") (declared-name "PressureSeat"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::Tire"))) (name "Tire") (declared-name "Tire") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (name "bead") (declared-name "bead") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Tire")))))
            (element (kind "action") (id (node (document "d0") (qualified-name "Wheel Package::Tire::mountTire"))) (name "mountTire") (declared-name "mountTire") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Tire")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::TireBead"))) (name "TireBead") (declared-name "TireBead") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::TireMountingRim"))) (name "TireMountingRim") (declared-name "TireMountingRim") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (name "Wheel") (declared-name "Wheel") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (name "mountingHoles") (declared-name "mountingHoles") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (name "rim") (declared-name "rim") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 2) (upper 2) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (name "v") (declared-name "v") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (name "weight") (declared-name "weight") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 0) (upper 6) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (name "WheelAssembly") (declared-name "WheelAssembly") (declared)
          (contains
            (element (kind "connection") (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (name "_connection") (declared-name "_connection") (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WheelAssembly")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (name "t") (declared-name "t") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (name "bead") (declared-name "bead") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Wheel Package::Tire")))))
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (name "w") (declared-name "w") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (name "rim") (declared-name "rim") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
              )
            )
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (name "hub") (declared-name "hub") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (name "lugBoltJoints") (declared-name "lugBoltJoints") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 5) (upper 5) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (composite true) (reference false) (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor"))) (name "WirelessTirePressureMonitor") (declared-name "WirelessTirePressureMonitor") (declared)
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor::transmitPressure"))) (name "transmitPressure") (declared-name "transmitPressure") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor")))))
          )
        )
        (element (kind "documentation") (id (node (document "d0") (qualified-name "Wheel Package::_documentation"))) (name ""))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::_documentation"))) (to (node (document "d0") (qualified-name "Wheel Package"))))
    (connection (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (to (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (connect (source-expression "t::bead") (target-expression "w::rim") (container-prefix "Wheel Package::WheelAssembly")))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (to (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (to (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (to (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (to (node (document "d0") (qualified-name "Wheel Package::TireBead"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (to (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (to (node (document "d0") (qualified-name "Wheel Package::TireMountingRim"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (to (node (document "d0") (qualified-name "Wheel Package::InflationValve"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (to (node (document "d0") (qualified-name "Wheel Package::BalanceWeight"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (to (node (document "d0") (qualified-name "Wheel Package::PressureSeat"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (to (node (document "d0") (qualified-name "Wheel Package::Tire"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (to (node (document "d0") (qualified-name "Wheel Package::Wheel"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (to (node (document "d0") (qualified-name "Wheel Package::Hub"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (to (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (to (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
