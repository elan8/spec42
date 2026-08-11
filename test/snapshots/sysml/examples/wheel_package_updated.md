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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "wheel_package_updated.md"
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
        (range (start 10 1) (end 10 47))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 16 2) (end 16 34))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 27 2) (end 27 23))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 27 2) (end 27 23))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 32 2) (end 32 44))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "sysml")
        (range (start 37 2) (end 37 20))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 37 2) (end 37 20))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 52 2) (end 52 26))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 56 2) (end 56 50))
      )
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 63 2) (end 63 50))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 83 3) (end 83 78))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 83 3) (end 83 78))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "537ad53749c7ad7a92893be5a8761429467c509b6340db557cfe206647f20c7d") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated"))) (kind "package") (name "Wheel Package - Updated") (declared-name "Wheel Package - Updated") (range (start (line 0) (character 0)) (end (line 0) (character 1752))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 6) (character 1)) (end (line 6) (character 23))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 6) (character 16)) (end (line 6) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (kind "part def") (name "BalanceWeight") (declared-name "BalanceWeight") (range (start (line 49) (character 1)) (end (line 49) (character 24))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::BandMount"))) (kind "connection def") (name "BandMount") (declared-name "BandMount") (range (start (line 36) (character 1)) (end (line 36) (character 88))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::Hub"))) (kind "part def") (name "Hub") (declared-name "Hub") (range (start (line 60) (character 1)) (end (line 60) (character 14))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::InflationValve"))) (kind "part def") (name "InflationValve") (declared-name "InflationValve") (range (start (line 47) (character 1)) (end (line 47) (character 25))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (kind "part def") (name "LugBoltJoint") (declared-name "LugBoltJoint") (range (start (line 55) (character 1)) (end (line 55) (character 76))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (kind "part def") (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (range (start (line 51) (character 1)) (end (line 51) (character 59))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (kind "part def") (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (range (start (line 62) (character 1)) (end (line 62) (character 85))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::PressureSeat"))) (kind "connection def") (name "PressureSeat") (declared-name "PressureSeat") (range (start (line 26) (character 1)) (end (line 26) (character 82))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (range (start (line 19) (character 1)) (end (line 19) (character 108))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::Tire::mountTire"))) (kind "action") (name "mountTire") (declared-name "mountTire") (range (start (line 21) (character 2)) (end (line 21) (character 19))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::Tire"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::TireBead"))) (kind "part def") (name "TireBead") (declared-name "TireBead") (range (start (line 24) (character 1)) (end (line 24) (character 19))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (kind "part def") (name "TireMountingRim") (declared-name "TireMountingRim") (range (start (line 45) (character 1)) (end (line 45) (character 26))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 31) (character 1)) (end (line 31) (character 63))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (kind "part def") (name "WheelAssembly") (declared-name "WheelAssembly") (range (start (line 15) (character 1)) (end (line 15) (character 61))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (kind "part def") (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (range (start (line 14) (character 1)) (end (line 14) (character 27))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor"))) (kind "part def") (name "WirelessTirePressureMonitor") (declared-name "WirelessTirePressureMonitor") (range (start (line 41) (character 1)) (end (line 41) (character 92))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor::transmitPressure"))) (kind "action") (name "transmitPressure") (declared-name "transmitPressure") (range (start (line 42) (character 2)) (end (line 42) (character 26))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1752))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind "part") (name "wheelHubAssembly") (declared-name "wheelHubAssembly") (range (start (line 69) (character 1)) (end (line 69) (character 631))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelHubAssembly") (range (start (line 69) (character 24)) (end (line 69) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind "part") (name "hub") (declared-name "hub") (range (start (line 86) (character 2)) (end (line 86) (character 61))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Hub") (range (start (line 86) (character 12)) (end (line 86) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind "part") (name "h") (declared-name "h") (range (start (line 87) (character 3)) (end (line 87) (character 36))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltThreadableHole") (range (start (line 87) (character 11)) (end (line 87) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind "part") (name "lugBoltJoints") (declared-name "lugBoltJoints") (range (start (line 82) (character 2)) (end (line 82) (character 184))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltJoint") (range (start (line 82) (character 22)) (end (line 82) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 70) (character 2)) (end (line 70) (character 338))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelAssembly") (range (start (line 70) (character 14)) (end (line 70) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind "part") (name "t") (declared-name "t") (range (start (line 71) (character 3)) (end (line 71) (character 57))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire") (range (start (line 71) (character 11)) (end (line 71) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind "part") (name "bead") (declared-name "bead") (range (start (line 72) (character 4)) (end (line 72) (character 28))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireBead") (range (start (line 72) (character 16)) (end (line 72) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind "part") (name "w") (declared-name "w") (range (start (line 74) (character 3)) (end (line 74) (character 181))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 74) (character 11)) (end (line 74) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind "part") (name "mountingHoles") (declared-name "mountingHoles") (range (start (line 78) (character 4)) (end (line 78) (character 48))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltMountingHole") (range (start (line 78) (character 25)) (end (line 78) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind "part") (name "rim") (declared-name "rim") (range (start (line 75) (character 4)) (end (line 75) (character 34))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireMountingRim") (range (start (line 75) (character 15)) (end (line 75) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind "part") (name "v") (declared-name "v") (range (start (line 76) (character 4)) (end (line 76) (character 31))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "InflationValve") (range (start (line 76) (character 13)) (end (line 76) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind "part") (name "weight") (declared-name "weight") (range (start (line 77) (character 4)) (end (line 77) (character 38))) (parent (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "BalanceWeight") (range (start (line 77) (character 18)) (end (line 77) (character 31)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 6) (character 16)) (end (line 6) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelHubAssembly") (range (start (line 69) (character 24)) (end (line 69) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "Hub") (range (start (line 86) (character 12)) (end (line 86) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::Hub")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltThreadableHole") (range (start (line 87) (character 11)) (end (line 87) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltJoint") (range (start (line 82) (character 22)) (end (line 82) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltJoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelAssembly") (range (start (line 70) (character 14)) (end (line 70) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (range (start (line 71) (character 11)) (end (line 71) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)) (authored-target "TireBead") (range (start (line 72) (character 16)) (end (line 72) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::TireBead")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 74) (character 11)) (end (line 74) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltMountingHole") (range (start (line 78) (character 25)) (end (line 78) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltMountingHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)) (authored-target "TireMountingRim") (range (start (line 75) (character 15)) (end (line 75) (character 30))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::TireMountingRim")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind featureTyping) (ordinal 0)) (authored-target "InflationValve") (range (start (line 76) (character 13)) (end (line 76) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::InflationValve")))))
    (reference (id (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind featureTyping) (ordinal 0)) (authored-target "BalanceWeight") (range (start (line 77) (character 18)) (end (line 77) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Wheel Package - Updated::BalanceWeight")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::Hub"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::TireBead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::InflationValve"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (target (node (document "d0") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind featureTyping) (ordinal 0)))
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
        (source (document "d0") (qualified-name "Wheel Package - Updated::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 6 16) (end 6 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 86 12) (end 86 15)) (probe (position 86 12))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))
        (kind featureTyping) (ordinal 0) (authored-target "Hub")
        (range (start 86 12) (end 86 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::Hub") (range (start 60 1) (end 60 14)))
        )
      )
    )
    (query (range (start 71 11) (end 71 15)) (probe (position 71 11))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))
        (kind featureTyping) (ordinal 0) (authored-target "Tire")
        (range (start 71 11) (end 71 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::Tire") (range (start 19 1) (end 19 108)))
        )
      )
    )
    (query (range (start 74 11) (end 74 16)) (probe (position 74 11))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 74 11) (end 74 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::Wheel") (range (start 31 1) (end 31 63)))
        )
      )
    )
    (query (range (start 72 16) (end 72 24)) (probe (position 72 16))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))
        (kind featureTyping) (ordinal 0) (authored-target "TireBead")
        (range (start 72 16) (end 72 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::TireBead") (range (start 24 1) (end 24 19)))
        )
      )
    )
    (query (range (start 82 22) (end 82 34)) (probe (position 82 22))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltJoint")
        (range (start 82 22) (end 82 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::LugBoltJoint") (range (start 55 1) (end 55 76)))
        )
      )
    )
    (query (range (start 70 14) (end 70 27)) (probe (position 70 14))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))
        (kind featureTyping) (ordinal 0) (authored-target "WheelAssembly")
        (range (start 70 14) (end 70 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::WheelAssembly") (range (start 15 1) (end 15 61)))
        )
      )
    )
    (query (range (start 77 18) (end 77 31)) (probe (position 77 18))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))
        (kind featureTyping) (ordinal 0) (authored-target "BalanceWeight")
        (range (start 77 18) (end 77 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::BalanceWeight") (range (start 49 1) (end 49 24)))
        )
      )
    )
    (query (range (start 76 13) (end 76 27)) (probe (position 76 13))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))
        (kind featureTyping) (ordinal 0) (authored-target "InflationValve")
        (range (start 76 13) (end 76 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::InflationValve") (range (start 47 1) (end 47 25)))
        )
      )
    )
    (query (range (start 75 15) (end 75 30)) (probe (position 75 15))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))
        (kind featureTyping) (ordinal 0) (authored-target "TireMountingRim")
        (range (start 75 15) (end 75 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::TireMountingRim") (range (start 45 1) (end 45 26)))
        )
      )
    )
    (query (range (start 69 24) (end 69 40)) (probe (position 69 24))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "WheelHubAssembly")
        (range (start 69 24) (end 69 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::WheelHubAssembly") (range (start 14 1) (end 14 27)))
        )
      )
    )
    (query (range (start 78 25) (end 78 44)) (probe (position 78 25))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltMountingHole")
        (range (start 78 25) (end 78 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::LugBoltMountingHole") (range (start 51 1) (end 51 59)))
        )
      )
    )
    (query (range (start 87 11) (end 87 32)) (probe (position 87 11))
      (reference
        (source (document "d0") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltThreadableHole")
        (range (start 87 11) (end 87 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole") (range (start 62 1) (end 62 85)))
        )
      )
    )
  )
)
~~~
