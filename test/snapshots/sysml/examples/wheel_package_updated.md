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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 12) (end 10 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 10 20) (end 10 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 20 22) (end 20 42))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 14) (end 32 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 33 11) (end 33 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 17) (end 52 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 12) (end 56 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 57 17) (end 57 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 17) (end 63 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 64 16) (end 64 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 80 37) (end 80 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:40317cc79cc2865c6d609256dc60a7107977290cc80c6cfecefbd5ca066947c8") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated"))) (kind package) (membership (kind owning) (visibility default)) (documentation (doc (text "\n\t * Example from the SysML 1.6 spec, subclause 8.4.1 Wheel Hub Assembly.\n\t "))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BandMount"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Hub"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::InflationValve"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint::boltTension"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "force"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint::torque"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::torque"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole::lugBoltSize"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "length"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole::lugBoltSize"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "length"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole::threadSize"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "length"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire::mountTire"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire::tireSpecification"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ScalarValues::String"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireBead"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel::diameter"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "length"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel::width"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "length"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly::inflationPressure"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "pressure"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WirelessTirePressureMonitor::transmitPressure"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "force")) (expressionOperand (reference "length"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelHubAssembly"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hub"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltThreadableHole"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltJoint"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltMountingHole")) (subsetting (reference "wheel::w::mountingHoles"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind ref) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltThreadableHole")) (subsetting (reference "hub::h"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelAssembly"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PressureSeat")) (memberAccessOperand (reference "t::bead")) (memberAccessOperand (reference "w::rim"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tire"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireBead"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltMountingHole"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireMountingRim"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "InflationValve"))))
    (declaration (id (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 6))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "BalanceWeight"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint::boltTension"))) (kind subsetting) (ordinal 0))
      (authored-target "force")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint::torque"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::torque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole::lugBoltSize"))) (kind subsetting) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole::lugBoltSize"))) (kind subsetting) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole::threadSize"))) (kind subsetting) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire::tireSpecification"))) (kind featureTyping) (ordinal 0))
      (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel::diameter"))) (kind subsetting) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel::width"))) (kind subsetting) (ordinal 0))
      (authored-target "length")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly::inflationPressure"))) (kind subsetting) (ordinal 0))
      (authored-target "pressure")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (kind expressionOperand) (ordinal 0))
      (authored-target "force")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (kind expressionOperand) (ordinal 1))
      (authored-target "length")
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
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltMountingHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind subsetting) (ordinal 0))
      (authored-target "wheel::w::mountingHoles")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltThreadableHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind subsetting) (ordinal 0))
      (authored-target "hub::h")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind featureTyping) (ordinal 0))
      (authored-target "PressureSeat")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat")))))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "t::bead")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "w::rim")
      (outcome (status unresolved)))
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
    (relationship (kind subsetting) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly::inflationPressure"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly::inflationPressure"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelHubAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Hub"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireBead"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::InflationValve"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::BalanceWeight"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::weight"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 6 16) (end 6 22)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (anonymous (kind import) (ordinal 0)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 57 17) (end 57 22)) (probe (position 57 17))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint::boltTension"))) (kind subsetting) (ordinal 0) (authored-target "force")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 56 12) (end 56 23)) (probe (position 56 12))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltJoint::torque"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::torque")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 52 17) (end 52 23)) (probe (position 52 17))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole::lugBoltSize"))) (kind subsetting) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 63 17) (end 63 23)) (probe (position 63 17))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole::lugBoltSize"))) (kind subsetting) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 64 16) (end 64 22)) (probe (position 64 16))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole::threadSize"))) (kind subsetting) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 20 22) (end 20 42)) (probe (position 20 22))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Tire::tireSpecification"))) (kind featureTyping) (ordinal 0) (authored-target "ScalarValues::String")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 32 14) (end 32 20)) (probe (position 32 14))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel::diameter"))) (kind subsetting) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 33 11) (end 33 17)) (probe (position 33 11))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::Wheel::width"))) (kind subsetting) (ordinal 0) (authored-target "length")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 16 23) (end 16 31)) (probe (position 16 23))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly::inflationPressure"))) (kind subsetting) (ordinal 0) (authored-target "pressure")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 10 12) (end 10 17)) (probe (position 10 12))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (kind expressionOperand) (ordinal 0) (authored-target "force")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 10 20) (end 10 26)) (probe (position 10 20))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::pressure"))) (kind expressionOperand) (ordinal 1) (authored-target "length")
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
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 83 21) (end 83 40)) (probe (position 83 21))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltMountingHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltMountingHole")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 83 52) (end 83 73)) (probe (position 83 52))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::mountingHole"))) (kind subsetting) (ordinal 0) (authored-target "wheel::w::mountingHoles")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel::w::mountingHoles")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 84 21) (end 84 42)) (probe (position 84 21))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltThreadableHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::LugBoltThreadableHole")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 84 54) (end 84 59)) (probe (position 84 54))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::lugBoltJoints::threadedHole"))) (kind subsetting) (ordinal 0) (authored-target "hub::h")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::hub::h")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 70 14) (end 70 27)) (probe (position 70 14))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "WheelAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::WheelAssembly")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 80 16) (end 80 28)) (probe (position 80 16))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind featureTyping) (ordinal 0) (authored-target "PressureSeat")
      (outcome (status resolved) (target (node (document "memory://snapshot/wheel_package_updated.md") (qualified-name "Wheel Package - Updated::PressureSeat")))))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 80 37) (end 80 43)) (probe (position 80 37))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "t::bead")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/wheel_package_updated.md") (range (start 80 47) (end 80 52)) (probe (position 80 47))
    (reference (id (source (node (document "memory://snapshot/wheel_package_updated.md") (path (named (kind package) (name "Wheel Package - Updated")) (named (kind part) (name "wheelHubAssembly")) (named (kind part) (name "wheel")) (anonymous (kind connection) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 1) (authored-target "w::rim")
      (outcome (status unresolved)))
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
