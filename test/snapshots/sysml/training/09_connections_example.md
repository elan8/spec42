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
  (document "09_connections_example.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "missing_semicolon")
        (source "sysml")
        (range (start 28 3) (end 28 34))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 28 3) (end 28 34))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "3abef445b1d3de9cb72c75a8e5f18b5ab323fa5aa2fcc9117419c90e593e4731") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Connections Example"))) (kind "package") (name "Connections Example") (declared-name "Connections Example"))
    (element (id (node (document "d0") (qualified-name "Connections Example::Hub"))) (kind "part def") (name "Hub") (declared-name "Hub") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::LugBoltJoint"))) (kind "part def") (name "LugBoltJoint") (declared-name "LugBoltJoint") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::LugBoltMountingHole"))) (kind "part def") (name "LugBoltMountingHole") (declared-name "LugBoltMountingHole") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::LugBoltThreadableHole"))) (kind "part def") (name "LugBoltThreadableHole") (declared-name "LugBoltThreadableHole") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::PressureSeat"))) (kind "connection def") (name "PressureSeat") (declared-name "PressureSeat") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::PressureSeat::bead"))) (kind "interface end") (name "bead") (declared-name "bead") (parent (node (document "d0") (qualified-name "Connections Example::PressureSeat"))) (authored (relationships (typing (reference "TireBead")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind "interface end") (name "mountingRim") (declared-name "mountingRim") (parent (node (document "d0") (qualified-name "Connections Example::PressureSeat"))) (authored (relationships (typing (reference "TireMountingRim")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::Tire"))) (kind "part def") (name "Tire") (declared-name "Tire") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::TireBead"))) (kind "part def") (name "TireBead") (declared-name "TireBead") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))) (kind "part def") (name "TireMountingRim") (declared-name "TireMountingRim") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::WheelAssembly"))) (kind "part def") (name "WheelAssembly") (declared-name "WheelAssembly") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly"))) (kind "part def") (name "WheelHubAssembly") (declared-name "WheelHubAssembly") (parent (node (document "d0") (qualified-name "Connections Example"))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind "part") (name "wheelHubAssembly") (declared-name "wheelHubAssembly") (parent (node (document "d0") (qualified-name "Connections Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelHubAssembly")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind "part") (name "hub") (declared-name "hub") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "Hub")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind "part") (name "h") (declared-name "h") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltThreadableHole")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind "part") (name "lugBoltJoints") (declared-name "lugBoltJoints") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltJoint")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelAssembly")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind "part") (name "t") (declared-name "t") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Tire")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind "part") (name "bead") (declared-name "bead") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireBead")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind "part") (name "w") (declared-name "w") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind "part") (name "mountingHoles") (declared-name "mountingHoles") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBoltMountingHole")))))
    (element (id (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind "part") (name "rim") (declared-name "rim") (parent (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "TireMountingRim")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::PressureSeat::bead"))) (kind featureTyping) (ordinal 0)) (authored-target "TireBead") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::TireBead")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind featureTyping) (ordinal 0)) (authored-target "TireMountingRim") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::TireMountingRim")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelHubAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind connectionSource) (ordinal 0)) (authored-target "lugBoltJoints") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind connectionSource) (ordinal 1)) (authored-target "lugBoltJoints") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind connectionTarget) (ordinal 0)) (authored-target "wheel::w::mountingHoles") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind connectionTarget) (ordinal 1)) (authored-target "hub::h") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)) (authored-target "Hub") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::Hub")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltThreadableHole") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::LugBoltThreadableHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltJoint") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::LugBoltJoint")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelAssembly") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::WheelAssembly")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)) (authored-target "Tire") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::Tire")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)) (authored-target "TireBead") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::TireBead")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBoltMountingHole") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::LugBoltMountingHole")))))
    (reference (id (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)) (authored-target "TireMountingRim") (outcome (status resolved) (target (node (document "d0") (qualified-name "Connections Example::TireMountingRim")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::PressureSeat::bead"))) (target (node (document "d0") (qualified-name "Connections Example::TireBead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::PressureSeat::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (target (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::PressureSeat::mountingRim"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (target (node (document "d0") (qualified-name "Connections Example::WheelHubAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (target (node (document "d0") (qualified-name "Connections Example::Hub"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (target (node (document "d0") (qualified-name "Connections Example::LugBoltThreadableHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (target (node (document "d0") (qualified-name "Connections Example::LugBoltJoint"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (target (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind connectionSource) (ordinal 1)) (expression (kind connection) (source "lugBoltJoints") (target "hub::h")))
    (relationship (kind connection) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))) (target (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))) (kind connectionSource) (ordinal 0)) (expression (kind connection) (source "lugBoltJoints") (target "wheel::w::mountingHoles")))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (target (node (document "d0") (qualified-name "Connections Example::WheelAssembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (target (node (document "d0") (qualified-name "Connections Example::Tire"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (target (node (document "d0") (qualified-name "Connections Example::TireBead"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (target (node (document "d0") (qualified-name "Connections Example::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (target (node (document "d0") (qualified-name "Connections Example::LugBoltMountingHole"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (target (node (document "d0") (qualified-name "Connections Example::TireMountingRim"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 34 13) (end 34 16)) (probe (position 34 13))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub"))
        (kind featureTyping) (ordinal 0) (authored-target "Hub")
        (range (start 34 13) (end 34 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::Hub") (range (start 9 1) (end 9 14)))
        )
      )
    )
    (query (range (start 21 12) (end 21 16)) (probe (position 21 12))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t"))
        (kind featureTyping) (ordinal 0) (authored-target "Tire")
        (range (start 21 12) (end 21 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::Tire") (range (start 4 1) (end 4 15)))
        )
      )
    )
    (query (range (start 24 11) (end 24 16)) (probe (position 24 11))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 24 11) (end 24 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::Wheel") (range (start 6 1) (end 6 16)))
        )
      )
    )
    (query (range (start 38 38) (end 38 43)) (probe (position 38 38))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))
        (kind connectionTarget) (ordinal 1) (authored-target "hub::h")
        (range (start 38 38) (end 38 43))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h") (range (start 35 3) (end 35 37)))
        )
      )
    )
    (query (range (start 22 16) (end 22 24)) (probe (position 22 16))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::t::bead"))
        (kind featureTyping) (ordinal 0) (authored-target "TireBead")
        (range (start 22 16) (end 22 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::TireBead") (range (start 5 1) (end 5 19)))
        )
      )
    )
    (query (range (start 33 23) (end 33 35)) (probe (position 33 23))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltJoint")
        (range (start 33 23) (end 33 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::LugBoltJoint") (range (start 11 1) (end 11 23)))
        )
      )
    )
    (query (range (start 20 15) (end 20 28)) (probe (position 20 15))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel"))
        (kind featureTyping) (ordinal 0) (authored-target "WheelAssembly")
        (range (start 20 15) (end 20 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::WheelAssembly") (range (start 3 1) (end 3 24)))
        )
      )
    )
    (query (range (start 37 16) (end 37 30)) (probe (position 37 16))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))
        (kind connectionSource) (ordinal 0) (authored-target "lugBoltJoints")
        (range (start 37 16) (end 37 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints") (range (start 33 2) (end 33 42)))
        )
      )
    )
    (query (range (start 38 16) (end 38 30)) (probe (position 38 16))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))
        (kind connectionSource) (ordinal 1) (authored-target "lugBoltJoints")
        (range (start 38 16) (end 38 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::wheelHubAssembly::lugBoltJoints") (range (start 33 2) (end 33 42)))
        )
      )
    )
    (query (range (start 25 15) (end 25 30)) (probe (position 25 15))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::rim"))
        (kind featureTyping) (ordinal 0) (authored-target "TireMountingRim")
        (range (start 25 15) (end 25 30))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::TireMountingRim") (range (start 7 1) (end 7 26)))
        )
      )
    )
    (query (range (start 18 25) (end 18 41)) (probe (position 18 25))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))
        (kind featureTyping) (ordinal 0) (authored-target "WheelHubAssembly")
        (range (start 18 25) (end 18 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::WheelHubAssembly") (range (start 2 1) (end 2 27)))
        )
      )
    )
    (query (range (start 26 25) (end 26 44)) (probe (position 26 25))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltMountingHole")
        (range (start 26 25) (end 26 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::LugBoltMountingHole") (range (start 8 1) (end 8 30)))
        )
      )
    )
    (query (range (start 35 12) (end 35 33)) (probe (position 35 12))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly::hub::h"))
        (kind featureTyping) (ordinal 0) (authored-target "LugBoltThreadableHole")
        (range (start 35 12) (end 35 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::LugBoltThreadableHole") (range (start 10 1) (end 10 32)))
        )
      )
    )
    (query (range (start 37 38) (end 37 59)) (probe (position 37 38))
      (reference
        (source (document "d0") (qualified-name "Connections Example::wheelHubAssembly"))
        (kind connectionTarget) (ordinal 0) (authored-target "wheel::w::mountingHoles")
        (range (start 37 38) (end 37 59))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Connections Example::wheelHubAssembly::wheel::w::mountingHoles") (range (start 26 4) (end 26 48)))
        )
      )
    )
  )
)
~~~
