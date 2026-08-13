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
  (document "memory://snapshot/wheel_package_updated.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 6 16) (end 6 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 10 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 32))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 43))
      )
      (diagnostic
        (severity error)
        (code "recovered_connection_def_body_element")
        (source "parser")
        (range (start 27 2) (end 28 2))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 27 2) (end 28 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 32 2) (end 32 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 33 2) (end 33 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 52 2) (end 52 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 56 2) (end 56 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 57 2) (end 57 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 63 2) (end 63 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 64 2) (end 64 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_connection_definition_member")
        (source "semantic")
        (range (start 80 37) (end 80 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_connection_definition_member")
        (source "semantic")
        (range (start 80 47) (end 80 52))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:40317cc79cc2865c6d609256dc60a7107977290cc80c6cfecefbd5ca066947c8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BandMount"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Hub"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::InflationValve"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire::mountTire"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireBead"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor::transmitPressure"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelHubAssembly"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hub"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltThreadableHole"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltJoint"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelAssembly"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PressureSeat"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tire"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireBead"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltMountingHole"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireMountingRim"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InflationValve"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BalanceWeight"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelHubAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelHubAssembly")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Hub")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltThreadableHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltJoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "PressureSeat")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireBead")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireBead")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltMountingHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireMountingRim")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireMountingRim")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "InflationValve")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::InflationValve")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind featureTyping) (ordinal 0))
      (authored-target "BalanceWeight")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BalanceWeight")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Hub"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireBead"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::InflationValve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 6 16) (end 6 22)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 69 24) (end 69 40)) (probe (position 69 24))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "WheelHubAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelHubAssembly")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 86 12) (end 86 15)) (probe (position 86 12))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0) (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Hub")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 87 11) (end 87 32)) (probe (position 87 11))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltThreadableHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 82 22) (end 82 34)) (probe (position 82 22))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltJoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 70 14) (end 70 27)) (probe (position 70 14))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "WheelAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 80 16) (end 80 28)) (probe (position 80 16))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (anonymous (kind connection) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "PressureSeat")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 71 11) (end 71 15)) (probe (position 71 11))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0) (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 72 16) (end 72 24)) (probe (position 72 16))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0) (authored-target "TireBead")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireBead")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 74 11) (end 74 16)) (probe (position 74 11))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 78 25) (end 78 44)) (probe (position 78 25))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltMountingHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 75 15) (end 75 30)) (probe (position 75 15))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0) (authored-target "TireMountingRim")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireMountingRim")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 76 13) (end 76 27)) (probe (position 76 13))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind featureTyping) (ordinal 0) (authored-target "InflationValve")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::InflationValve")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 77 18) (end 77 31)) (probe (position 77 18))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind featureTyping) (ordinal 0) (authored-target "BalanceWeight")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BalanceWeight")))))
  )
)
~~~
