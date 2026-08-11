# META
~~~ini
description=SysML Example (v1 Spec): Vehicle Decomposition - Updated
type=file
~~~
# SOURCE
~~~sysml
package 'Vehicle Decomposition - Updated' {
	doc
	/*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */
	
	// Blocks
	
	part def Vehicle;
	
	part def 'Chassis Assembly';
	
	part def Wheel;
	
	part def LugBolt;
	
	part def RollBar;
	part def HeavyRollBar :> RollBar;
	part def LightRollBar :> RollBar;
	
	part def Engine;
	
	part def Cylinder;
	
	// Parts
	
	part vehicle : Vehicle {
		part chs : 'Chassis Assembly'[1] {
			part rb : RollBar[0..1];
			part w : Wheel[4] {
				part lb : LugBolt[6..10];
			}
		}
		part eng: Engine[1] {
			part cyl : Cylinder[4..8];
		}
	}
	
	
	part 'vehicle model 1' :> vehicle {
		part redefines chs {
			part redefines rb : LightRollBar[0..1];
			part redefines w {
				part redefines lb;
			}
		}
		part redefines eng {
			part redefines cyl[4];
		}
		
		// Constrains total number of lugbolts.
		ref lugBolts[24] = chs.w.lb;
	}
	
	part 'vehicle model 2' :> vehicle {
		part redefines chs {
			part redefines rb[0];
			part redefines w {
				// Constrains number of lugbolts per wheel.
				part redefines lb[6..7];
			}
		}
		part redefines eng {
			part redefines cyl[6..8];
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "vehicle_decomposition_updated.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "sysml")
        (range (start 42 3) (end 42 46))
      )
      (diagnostic
        (severity warning)
        (code "recovery_cascade_suppressed")
        (source "sysml")
        (range (start 42 3) (end 42 46))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Vehicle Decomposition - Updated' {
    doc
    /*
	 * Example from the SysML 1.6 spec, subclause 8.4.5 Constraining Decomposition,
	 * updated for usage-focused approach.
	 */

    // Blocks

    part def Vehicle;

    part def 'Chassis Assembly';

    part def Wheel;

    part def LugBolt;

    part def RollBar;
    part def HeavyRollBar :> RollBar;
    part def LightRollBar :> RollBar;

    part def Engine;

    part def Cylinder;

    // Parts

    part vehicle : Vehicle {
        part chs : 'Chassis Assembly'[1] {
            part rb : RollBar[0..1];
            part w : Wheel[4] {
                part lb : LugBolt[6..10];
            }
        }
        part eng: Engine[1] {
            part cyl : Cylinder[4..8];
        }
    }

    part 'vehicle model 1' :> vehicle {
        part redefines chs {
            part redefines rb : LightRollBar[0..1];
            part redefines w {
                part redefines lb;
            }
        }
        part redefines eng {
            part redefines cyl[4];
        }

        // Constrains total number of lugbolts.
        ref lugBolts[24] = chs.w.lb;
    }

    part 'vehicle model 2' :> vehicle {
        part redefines chs {
            part redefines rb[0];
            part redefines w {
                // Constrains number of lugbolts per wheel.
                part redefines lb[6..7];
            }
        }
        part redefines eng {
            part redefines cyl[6..8];
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "0e0f33d6f1e96a7b24fc851a1d87fc050c7ed0d808c7fd7abf7b22f1e5220544") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (kind "package") (name "Vehicle Decomposition - Updated") (declared-name "Vehicle Decomposition - Updated") (range (start (line 0) (character 0)) (end (line 0) (character 1194))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (kind "part def") (name "Chassis Assembly") (declared-name "Chassis Assembly") (range (start (line 11) (character 1)) (end (line 11) (character 29))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 23) (character 1)) (end (line 23) (character 19))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 21) (character 1)) (end (line 21) (character 17))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind "part def") (name "HeavyRollBar") (declared-name "HeavyRollBar") (range (start (line 18) (character 1)) (end (line 18) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RollBar") (range (start (line 18) (character 26)) (end (line 18) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind "part def") (name "LightRollBar") (declared-name "LightRollBar") (range (start (line 19) (character 1)) (end (line 19) (character 34))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (authored (membership (kind Owning)) (relationships (specializes (reference "RollBar") (range (start (line 19) (character 26)) (end (line 19) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (kind "part def") (name "LugBolt") (declared-name "LugBolt") (range (start (line 15) (character 1)) (end (line 15) (character 18))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (kind "part def") (name "RollBar") (declared-name "RollBar") (range (start (line 17) (character 1)) (end (line 17) (character 18))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 9) (character 1)) (end (line 9) (character 18))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (kind "part def") (name "Wheel") (declared-name "Wheel") (range (start (line 13) (character 1)) (end (line 13) (character 16))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::_documentation"))) (kind "documentation") (name "") (range (start (line 0) (character 0)) (end (line 0) (character 1194))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 27) (character 1)) (end (line 27) (character 213))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 27) (character 16)) (end (line 27) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind "part") (name "vehicle model 1") (declared-name "vehicle model 1") (range (start (line 40) (character 1)) (end (line 40) (character 288))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 40) (character 27)) (end (line 40) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (kind "part") (name "chs") (range (start (line 41) (character 2)) (end (line 41) (character 119))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "chs") (range (start (line 41) (character 17)) (end (line 41) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (kind "part") (name "w") (range (start (line 43) (character 3)) (end (line 43) (character 49))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "w") (range (start (line 43) (character 18)) (end (line 43) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (kind "part") (name "lb") (range (start (line 44) (character 4)) (end (line 44) (character 22))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lb") (range (start (line 44) (character 19)) (end (line 44) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (kind "part") (name "eng") (range (start (line 47) (character 2)) (end (line 47) (character 52))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng") (range (start (line 47) (character 17)) (end (line 47) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (kind "part") (name "cyl") (range (start (line 48) (character 3)) (end (line 48) (character 25))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 48) (character 18)) (end (line 48) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind "part") (name "vehicle model 2") (declared-name "vehicle model 2") (range (start (line 55) (character 1)) (end (line 55) (character 251))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "vehicle") (range (start (line 55) (character 27)) (end (line 55) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (kind "part") (name "chs") (range (start (line 56) (character 2)) (end (line 56) (character 155))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "chs") (range (start (line 56) (character 17)) (end (line 56) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (kind "part") (name "rb") (range (start (line 57) (character 3)) (end (line 57) (character 24))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "rb") (range (start (line 57) (character 18)) (end (line 57) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (kind "part") (name "w") (range (start (line 58) (character 3)) (end (line 58) (character 103))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "w") (range (start (line 58) (character 18)) (end (line 58) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (kind "part") (name "lb") (range (start (line 60) (character 4)) (end (line 60) (character 28))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "lb") (range (start (line 60) (character 19)) (end (line 60) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (kind "part") (name "eng") (range (start (line 63) (character 2)) (end (line 63) (character 55))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "eng") (range (start (line 63) (character 17)) (end (line 63) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (kind "part") (name "cyl") (range (start (line 64) (character 3)) (end (line 64) (character 28))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 64) (character 18)) (end (line 64) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind "part") (name "chs") (declared-name "chs") (range (start (line 28) (character 2)) (end (line 28) (character 126))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Chassis Assembly") (range (start (line 28) (character 13)) (end (line 28) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind "part") (name "rb") (declared-name "rb") (range (start (line 29) (character 3)) (end (line 29) (character 27))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (authored (membership (kind Feature)) (relationships (typing (reference "RollBar") (range (start (line 29) (character 13)) (end (line 29) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind "part") (name "w") (declared-name "w") (range (start (line 30) (character 3)) (end (line 30) (character 57))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 30) (character 12)) (end (line 30) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind "part") (name "lb") (declared-name "lb") (range (start (line 31) (character 4)) (end (line 31) (character 29))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (authored (membership (kind Feature)) (relationships (typing (reference "LugBolt") (range (start (line 31) (character 14)) (end (line 31) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind "part") (name "eng") (declared-name "eng") (range (start (line 34) (character 2)) (end (line 34) (character 57))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 34) (character 12)) (end (line 34) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 35) (character 3)) (end (line 35) (character 29))) (parent (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 35) (character 14)) (end (line 35) (character 22)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind specialization) (ordinal 0)) (authored-target "RollBar") (range (start (line 18) (character 26)) (end (line 18) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind specialization) (ordinal 0)) (authored-target "RollBar") (range (start (line 19) (character 26)) (end (line 19) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 27) (character 16)) (end (line 27) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 40) (character 27)) (end (line 40) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (kind redefinition) (ordinal 0)) (authored-target "chs") (range (start (line 41) (character 17)) (end (line 41) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (kind redefinition) (ordinal 0)) (authored-target "w") (range (start (line 43) (character 18)) (end (line 43) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (kind redefinition) (ordinal 0)) (authored-target "lb") (range (start (line 44) (character 19)) (end (line 44) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 47) (character 17)) (end (line 47) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 48) (character 18)) (end (line 48) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle") (range (start (line 55) (character 27)) (end (line 55) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (kind redefinition) (ordinal 0)) (authored-target "chs") (range (start (line 56) (character 17)) (end (line 56) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (kind redefinition) (ordinal 0)) (authored-target "rb") (range (start (line 57) (character 18)) (end (line 57) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (kind redefinition) (ordinal 0)) (authored-target "w") (range (start (line 58) (character 18)) (end (line 58) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (kind redefinition) (ordinal 0)) (authored-target "lb") (range (start (line 60) (character 19)) (end (line 60) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (kind redefinition) (ordinal 0)) (authored-target "eng") (range (start (line 63) (character 17)) (end (line 63) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 64) (character 18)) (end (line 64) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind featureTyping) (ordinal 0)) (authored-target "Chassis Assembly") (range (start (line 28) (character 13)) (end (line 28) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind featureTyping) (ordinal 0)) (authored-target "RollBar") (range (start (line 29) (character 13)) (end (line 29) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 30) (character 12)) (end (line 30) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind featureTyping) (ordinal 0)) (authored-target "LugBolt") (range (start (line 31) (character 14)) (end (line 31) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LugBolt")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 34) (character 12)) (end (line 34) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 35) (character 14)) (end (line 35) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Cylinder")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::HeavyRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LightRollBar"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::chs::w::lb"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 1::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::rb"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::chs::w::lb"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle model 2::eng::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Chassis Assembly"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::RollBar"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::rb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Wheel"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::LugBolt"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::chs::w::lb"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (target (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Vehicle Decomposition - Updated::vehicle::eng::cyl"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
