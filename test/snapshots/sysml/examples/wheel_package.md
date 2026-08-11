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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "wheel_package.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 19))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 8 1) (end 8 34))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 13 3) (end 13 48))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 13 3) (end 13 48))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 20 2) (end 20 38))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 43 2) (end 43 23))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 43 2) (end 43 23))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 48 2) (end 48 46))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 58 2) (end 58 20))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 58 2) (end 58 20))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 73 2) (end 73 26))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 77 2) (end 77 54))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 89 2) (end 89 50))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "82cd23e6c4cd8a9fb819c89e0ca93097b71cdde130d348dd586e47453ef6a55b") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Wheel Package"))) (kind "package") (name "Wheel Package") (declared-name "Wheel Package"))
    (element (id (node (document "d0") (qualified-name "Wheel Package::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Wheel Package"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::BalanceWeight"))) (kind "part def") (name "BalanceWeight") (declared-name "BalanceWeight") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::BandMount"))) (kind "connection def") (name "BandMount") (declared-name "BandMount") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Hub"))) (kind "part def") (name "Hub") (declared-name "Hub") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (kind "part") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "Wheel Package::Hub"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltThreadableHole")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::InflationValve"))) (kind "part def") (name "InflationValve") (declared-name "InflationValve") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))) (kind "part def") (name "LugBoltJoint") (declared-name "LugBoltJoint") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint::mountingHole"))) (kind "opaque member") (name "mountingHole") (declared-name "mountingHole") (parent (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint::threadedHole"))) (kind "opaque member") (name "threadedHole") (declared-name "threadedHole") (parent (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole"))) (kind "part def") (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole"))) (kind "part def") (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::PressureSeat"))) (kind "connection def") (name "PressureSeat") (declared-name "PressureSeat") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (kind "part") (name "bead") (declared-name "bead") (parent (node (document "d0") (qualified-name "Wheel Package::Tire"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireBead")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Tire::mountTire"))) (kind "action") (name "mountTire") (declared-name "mountTire") (parent (node (document "d0") (qualified-name "Wheel Package::Tire"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::TireBead"))) (kind "part def") (name "TireBead") (declared-name "TireBead") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::TireMountingRim"))) (kind "part def") (name "TireMountingRim") (declared-name "TireMountingRim") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (kind "part") (name "mountingHoles") (declared-name "mountingHoles") (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltMountingHole")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (kind "part") (name "rim") (declared-name "rim") (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireMountingRim")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (kind "part") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "InflationValve")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (kind "part") (name "weight") (declared-name "weight") (parent (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "BalanceWeight")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind "part def") (name "WheelAssembly") (declared-name "WheelAssembly") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (kind "connection") (name "_connection") (declared-name "_connection") (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "PressureSeat")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (kind "part") (name "t") (declared-name "t") (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (kind "part") (name "bead") (declared-name "bead") (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Tire::bead")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (kind "part") (name "w") (declared-name "w") (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (kind "part") (name "rim") (declared-name "rim") (parent (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "Wheel::rim")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (kind "part def") (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (kind "part") (name "hub") (declared-name "hub") (parent (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Hub")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (kind "part") (name "lugBoltJoints") (declared-name "lugBoltJoints") (parent (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltJoint")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelAssembly")))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor"))) (kind "part def") (name "WirelessTirePressureMonitor") (declared-name "WirelessTirePressureMonitor") (parent (node (document "d0") (qualified-name "Wheel Package"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor::transmitPressure"))) (kind "action") (name "transmitPressure") (declared-name "transmitPressure") (parent (node (document "d0") (qualified-name "Wheel Package::WirelessTirePressureMonitor"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Wheel Package"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltThreadableHole") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (kind featureTyping) (ordinal 0)) (authored-target "TireBead") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::TireBead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltMountingHole") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (kind featureTyping) (ordinal 0)) (authored-target "TireMountingRim") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::TireMountingRim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (kind featureTyping) (ordinal 0)) (authored-target "InflationValve") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::InflationValve")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (kind featureTyping) (ordinal 0)) (authored-target "BalanceWeight") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::BalanceWeight")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "t::bead") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "w::rim") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (kind featureTyping) (ordinal 0)) (authored-target "PressureSeat") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::PressureSeat")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (kind redefinition) (ordinal 0)) (authored-target "Tire::bead") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Tire::bead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (kind redefinition) (ordinal 0)) (authored-target "Wheel::rim") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Wheel::rim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "Hub") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::Hub")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltJoint") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (target (node (document "d0") (qualified-name "Wheel Package::TireBead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (target (node (document "d0") (qualified-name "Wheel Package::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (target (node (document "d0") (qualified-name "Wheel Package::InflationValve"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (target (node (document "d0") (qualified-name "Wheel Package::BalanceWeight"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::Wheel::weight"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (target (node (document "d0") (qualified-name "Wheel Package::PressureSeat"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::_connection"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (target (node (document "d0") (qualified-name "Wheel Package::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (target (node (document "d0") (qualified-name "Wheel Package::Tire::bead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (kind redefinition) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "t::bead") (target "w::rim")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (target (node (document "d0") (qualified-name "Wheel Package::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (target (node (document "d0") (qualified-name "Wheel Package::Wheel::rim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (target (node (document "d0") (qualified-name "Wheel Package::Hub"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (target (node (document "d0") (qualified-name "Wheel Package::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (target (node (document "d0") (qualified-name "Wheel Package::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 16) (end 6 19)) (probe (position 6 16))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 6 16) (end 6 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 12) (end 16 15)) (probe (position 16 12))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::hub"))
        (kind featureTyping) (ordinal 0) (authored-target "Hub")
        (range (start 16 12) (end 16 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::Hub") (range (start 84 1) (end 84 54)))
        )
      )
    )
    (query (range (start 22 10) (end 22 14)) (probe (position 22 10))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelAssembly::t"))
        (kind featureTyping) (ordinal 0) (authored-target "Tire")
        (range (start 22 10) (end 22 14))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::Tire") (range (start 32 1) (end 32 116)))
        )
      )
    )
    (query (range (start 25 10) (end 25 15)) (probe (position 25 10))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelAssembly::w"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 25 10) (end 25 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::Wheel") (range (start 47 1) (end 47 211)))
        )
      )
    )
    (query (range (start 29 46) (end 29 51)) (probe (position 29 46))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelAssembly"))
        (kind connectionTarget) (ordinal 0) (authored-target "w::rim")
        (range (start 29 46) (end 29 51))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim") (range (start 26 3) (end 26 33)))
        )
      )
    )
    (query (range (start 29 36) (end 29 42)) (probe (position 29 36))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelAssembly"))
        (kind connectionSource) (ordinal 0) (authored-target "t::bead")
        (range (start 29 36) (end 29 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead") (range (start 23 3) (end 23 34)))
        )
      )
    )
    (query (range (start 35 14) (end 35 22)) (probe (position 35 14))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::Tire::bead"))
        (kind featureTyping) (ordinal 0) (authored-target "TireBead")
        (range (start 35 14) (end 35 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::TireBead") (range (start 40 1) (end 40 19)))
        )
      )
    )
    (query (range (start 23 23) (end 23 33)) (probe (position 23 23))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelAssembly::t::bead"))
        (kind redefinition) (ordinal 0) (authored-target "Tire::bead")
        (range (start 23 23) (end 23 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::Tire::bead") (range (start 35 2) (end 35 26)))
        )
      )
    )
    (query (range (start 26 22) (end 26 32)) (probe (position 26 22))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelAssembly::w::rim"))
        (kind redefinition) (ordinal 0) (authored-target "Wheel::rim")
        (range (start 26 22) (end 26 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::Wheel::rim") (range (start 51 2) (end 51 32)))
        )
      )
    )
    (query (range (start 12 22) (end 12 34)) (probe (position 12 22))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::lugBoltJoints"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltJoint")
        (range (start 12 22) (end 12 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::LugBoltJoint") (range (start 76 1) (end 76 169)))
        )
      )
    )
    (query (range (start 11 14) (end 11 27)) (probe (position 11 14))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::WheelHubAssembly::wheel"))
        (kind featureTyping) (ordinal 0) (authored-target "WheelAssembly")
        (range (start 11 14) (end 11 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::WheelAssembly") (range (start 19 1) (end 19 244)))
        )
      )
    )
    (query (range (start 53 16) (end 53 29)) (probe (position 53 16))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::Wheel::weight"))
        (kind featureTyping) (ordinal 0) (authored-target "BalanceWeight")
        (range (start 53 16) (end 53 29))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::BalanceWeight") (range (start 70 1) (end 70 24)))
        )
      )
    )
    (query (range (start 52 11) (end 52 25)) (probe (position 52 11))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::Wheel::v"))
        (kind featureTyping) (ordinal 0) (authored-target "InflationValve")
        (range (start 52 11) (end 52 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::InflationValve") (range (start 68 1) (end 68 25)))
        )
      )
    )
    (query (range (start 51 13) (end 51 28)) (probe (position 51 13))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::Wheel::rim"))
        (kind featureTyping) (ordinal 0) (authored-target "TireMountingRim")
        (range (start 51 13) (end 51 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::TireMountingRim") (range (start 66 1) (end 66 26)))
        )
      )
    )
    (query (range (start 54 23) (end 54 42)) (probe (position 54 23))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::Wheel::mountingHoles"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltMountingHole")
        (range (start 54 23) (end 54 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::LugBoltMountingHole") (range (start 72 1) (end 72 59)))
        )
      )
    )
    (query (range (start 85 10) (end 85 31)) (probe (position 85 10))
      (reference
        (source (document "d0") (qualified-name "Wheel Package::Hub::h"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltThreadableHole")
        (range (start 85 10) (end 85 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package::LugBoltThreadableHole") (range (start 88 1) (end 88 85)))
        )
      )
    )
  )
)
~~~
