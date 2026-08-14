# META
~~~ini
description=SysML Training 09 (Connections): Connections Example
type=file
~~~
# SOURCE
~~~sysml
package 'Connections Example' {
	
	part def WheelHubAssembly;
	part def WheelAssembly;
	part def Tire;
	part def TireBead;
	part def Wheel;
	part def TireMountingRim;
	part def LugBoltMountingHole;
	part def Hub;
	part def LugBoltThreadableHole;
	part def LugBoltJoint;
	
	connection def PressureSeat {
		end [1] part bead : TireBead;
		end [1] part mountingRim : TireMountingRim;
	}
	
	part wheelHubAssembly : WheelHubAssembly {
		
		part wheel : WheelAssembly[1] {
			part t : Tire[1] {
				part bead : TireBead[2];			
			}
			part w: Wheel[1] {
				part rim : TireMountingRim[2];
				part mountingHoles : LugBoltMountingHole[5];
			}						
			connection : PressureSeat 
				connect bead references t.bead 
				to mountingRim references w.rim;		
		}
		
		part lugBoltJoints : LugBoltJoint[0..5];
		part hub : Hub[1] {
			part h : LugBoltThreadableHole[5];
		}
		connect [0..1] lugBoltJoints to [1] wheel.w.mountingHoles;
		connect [0..1] lugBoltJoints to [1] hub.h;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/09_connections_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "parser")
        (range (start 28 3) (end 29 4))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "parser")
        (range (start 28 3) (end 29 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 38) (end 37 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 38 38) (end 38 43))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:457ea7f7731fde8b9fa2f3fa09ade51ab64e58eb3129433ad5cf3552353222f2") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Hub"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltJoint"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltMountingHole"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltThreadableHole"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat"))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::bead"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireBead"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind connection) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireMountingRim"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Tire"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Wheel"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelHubAssembly"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelHubAssembly"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "lugBoltJoints")) (memberAccessOperand (reference "wheel::w::mountingHoles"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "lugBoltJoints")) (memberAccessOperand (reference "hub::h"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Hub"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltThreadableHole"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 0) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltJoint"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelAssembly"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Tire"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireBead"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 5) (upper 5))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LugBoltMountingHole"))))
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 2) (upper 2))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TireMountingRim"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::bead"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireBead")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireMountingRim")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelHubAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelHubAssembly")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 0))
      (authored-target "lugBoltJoints")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind connectorEnd) (ordinal 0))
      (authored-target "lugBoltJoints")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "wheel::w::mountingHoles")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "hub::h")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0))
      (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Hub")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltThreadableHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltThreadableHole")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltJoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltJoint")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelAssembly")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0))
      (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Tire")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireBead")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Wheel")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0))
      (authored-target "LugBoltMountingHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltMountingHole")))))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0))
      (authored-target "TireMountingRim")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::bead"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelHubAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Hub"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Tire"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Wheel"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::bead")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::mountingRim")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelHubAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Hub")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub::h")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltThreadableHole")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltJoint")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelAssembly")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Tire")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Wheel")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltMountingHole")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim")))
      (supertype (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/09_connections_example.md") (range (start 14 22) (end 14 30)) (probe (position 14 22))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::bead"))) (kind featureTyping) (ordinal 0) (authored-target "TireBead")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 15 29) (end 15 44)) (probe (position 15 29))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind featureTyping) (ordinal 0) (authored-target "TireMountingRim")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 18 25) (end 18 41)) (probe (position 18 25))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly"))) (kind featureTyping) (ordinal 0) (authored-target "WheelHubAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelHubAssembly")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 37 17) (end 37 30)) (probe (position 37 17))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind connectorEnd) (ordinal 0) (authored-target "lugBoltJoints")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 38 17) (end 38 30)) (probe (position 38 17))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind connectorEnd) (ordinal 0) (authored-target "lugBoltJoints")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 37 38) (end 37 59)) (probe (position 37 38))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 0)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "wheel::w::mountingHoles")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 38 38) (end 38 43)) (probe (position 38 38))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (path (named (kind package) (name "Connections Example")) (named (kind part) (name "wheelHubAssembly")) (anonymous (kind bare-connect) (ordinal 1)))))) (kind memberAccessOperand) (ordinal 0) (authored-target "hub::h")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 34 13) (end 34 16)) (probe (position 34 13))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0) (authored-target "Hub")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Hub")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 35 12) (end 35 33)) (probe (position 35 12))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltThreadableHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltThreadableHole")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 33 23) (end 33 35)) (probe (position 33 23))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltJoint")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltJoint")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 20 15) (end 20 28)) (probe (position 20 15))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "WheelAssembly")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::WheelAssembly")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 21 12) (end 21 16)) (probe (position 21 12))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0) (authored-target "Tire")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Tire")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 22 16) (end 22 24)) (probe (position 22 16))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0) (authored-target "TireBead")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireBead")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 24 11) (end 24 16)) (probe (position 24 11))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::Wheel")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 26 25) (end 26 44)) (probe (position 26 25))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0) (authored-target "LugBoltMountingHole")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::LugBoltMountingHole")))))
  )
  (query (document "memory://snapshot/09_connections_example.md") (range (start 25 15) (end 25 30)) (probe (position 25 15))
    (reference (id (source (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0) (authored-target "TireMountingRim")
      (outcome (status resolved) (target (node (document "memory://snapshot/09_connections_example.md") (qualified-name "Connections Example::TireMountingRim")))))
  )
)
~~~
