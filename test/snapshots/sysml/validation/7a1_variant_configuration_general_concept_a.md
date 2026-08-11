# META
~~~ini
description=SysML Validation (07-Variant Configuration): 7a1-Variant Configuration - General Concept-a
type=file
~~~
# SOURCE
~~~sysml
package '7a1-Variant Configuration - General Concept-a' {
	
	action doX;
	action doY;
	
	part part1;
	part part2;
	part part3 {
		port p1;
	}
	part part4;
	part part5 {
		port p2;
		variation perform action doXorY {
			variant perform doX;
			variant perform doY;
		}
	}
	part part6;
	
	abstract part def SubsystemA {
		abstract part :>> part3[0..1];
	}
	
	abstract part def SubsystemB {
		abstract part :>> part5[1];		
	}
	
	part anyVehicleConfig {
		
		variation part subsystemA : SubsystemA {
			variant part subsystem1 : SubsystemA {
				part :>> part1[1];
				part :>> part2[1];
			}
			variant part subsystem2 : SubsystemA {
				part :>> part2[1];
				part :>> part3[1];
			}
		}

		variation part subsystemB : SubsystemB {
			variant part subsystem3 : SubsystemB {
				part :>> part4[1];
				part :>> part5[1];
			}
			variant part subsystem4 : SubsystemB {
				part :>> part5[1];
				part :>> part6[1];
			}
		}
		
		connect [0..1] subsystemA.part3.p1 to [1] subsystemB.part5.p2;
		
		assert constraint {
			subsystemA != subsystemA::subsystem2 | 
			subsystemB == subsystemB::subsystem3
		}
		
	}
	
	part vehicleConfigA :> anyVehicleConfig {		
		part :>> subsystemA = subsystemA::subsystem1;
		part :>> subsystemB = subsystemB::subsystem3 {
			part :>> part5 {
				perform action :>> doXorY = doX;
			}
		}
	}
	
	part VehicleConfigB :> anyVehicleConfig {
		part :>> subsystemA = subsystemA::subsystem2;
		part :>> subsystemB = subsystemB::subsystem4 {
			part :>> part5 {
				perform action :>> doXorY = doY;
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "7a1_variant_configuration_general_concept_a.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 10 1) (end 10 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 18 1) (end 18 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 16) (end 52 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 52 44) (end 52 63))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '7a1-Variant Configuration - General Concept-a' {

    action doX;
    action doY;

    part part1;
    part part2;
    part part3 {
        port p1;
    }
    part part4;
    part part5 {
        port p2;
        variation perform action doXorY {
            variant perform doX;
            variant perform doY;
        }
    }
    part part6;

    abstract part def SubsystemA {
        abstract part :>> part3[0..1];
    }

    abstract part def SubsystemB {
        abstract part :>> part5[1];
    }

    part anyVehicleConfig {

        variation part subsystemA : SubsystemA {
            variant part subsystem1 : SubsystemA {
                part :>> part1[1];
                part :>> part2[1];
            }
            variant part subsystem2 : SubsystemA {
                part :>> part2[1];
                part :>> part3[1];
            }
        }

        variation part subsystemB : SubsystemB {
            variant part subsystem3 : SubsystemB {
                part :>> part4[1];
                part :>> part5[1];
            }
            variant part subsystem4 : SubsystemB {
                part :>> part5[1];
                part :>> part6[1];
            }
        }

        connect [0..1] subsystemA.part3.p1 to [1] subsystemB.part5.p2;

        assert constraint {
            subsystemA != subsystemA::subsystem2 |
            subsystemB == subsystemB::subsystem3
        }

    }

    part vehicleConfigA :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem1;
        part :>> subsystemB = subsystemB::subsystem3 {
            part :>> part5 {
                perform action :>> doXorY = doX;
            }
        }
    }

    part VehicleConfigB :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem2;
        part :>> subsystemB = subsystemB::subsystem4 {
            part :>> part5 {
                perform action :>> doXorY = doY;
            }
        }
    }
}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "ab03ac47073692081af06d9aea3296883a426af7b093db109c16fbb6165d7fff") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (kind "package") (name "7a1-Variant Configuration - General Concept-a") (declared-name "7a1-Variant Configuration - General Concept-a") (range (start (line 0) (character 0)) (end (line 0) (character 1531))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (kind "part def") (name "SubsystemA") (declared-name "SubsystemA") (range (start (line 20) (character 1)) (end (line 20) (character 67))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (kind "part") (name "part3") (range (start (line 21) (character 2)) (end (line 21) (character 32))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part3") (range (start (line 21) (character 20)) (end (line 21) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (kind "part def") (name "SubsystemB") (declared-name "SubsystemB") (range (start (line 24) (character 1)) (end (line 24) (character 66))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (kind "part") (name "part5") (range (start (line 25) (character 2)) (end (line 25) (character 29))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 25) (character 20)) (end (line 25) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind "part") (name "VehicleConfigB") (declared-name "VehicleConfigB") (range (start (line 70) (character 1)) (end (line 70) (character 208))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig") (range (start (line 70) (character 24)) (end (line 70) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (kind "part") (name "subsystemA") (range (start (line 71) (character 2)) (end (line 71) (character 47))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA") (range (start (line 71) (character 11)) (end (line 71) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (kind "part") (name "subsystemB") (range (start (line 72) (character 2)) (end (line 72) (character 114))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB") (range (start (line 72) (character 11)) (end (line 72) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind "part") (name "part5") (range (start (line 73) (character 3)) (end (line 73) (character 61))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 73) (character 12)) (end (line 73) (character 17)))) (perform (reference "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::") (range none)))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::"))) (kind "action") (name "") (range (start (line 74) (character 4)) (end (line 74) (character 36))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind "part") (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (range (start (line 28) (character 1)) (end (line 28) (character 680))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind "part") (name "subsystemA") (declared-name "subsystemA") (range (start (line 30) (character 2)) (end (line 30) (character 232))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemA") (range (start (line 30) (character 30)) (end (line 30) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (kind "part") (name "subsystem1") (declared-name "subsystem1") (range (start (line 31) (character 11)) (end (line 31) (character 92))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemA") (range (start (line 31) (character 29)) (end (line 31) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind "part") (name "part1") (range (start (line 32) (character 4)) (end (line 32) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part1") (range (start (line 32) (character 13)) (end (line 32) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind "part") (name "part2") (range (start (line 33) (character 4)) (end (line 33) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2") (range (start (line 33) (character 13)) (end (line 33) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (kind "part") (name "subsystem2") (declared-name "subsystem2") (range (start (line 35) (character 11)) (end (line 35) (character 92))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemA") (range (start (line 35) (character 29)) (end (line 35) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind "part") (name "part2") (range (start (line 36) (character 4)) (end (line 36) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2") (range (start (line 36) (character 13)) (end (line 36) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind "part") (name "part3") (range (start (line 37) (character 4)) (end (line 37) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part3") (range (start (line 37) (character 13)) (end (line 37) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind "part") (name "subsystemB") (declared-name "subsystemB") (range (start (line 41) (character 2)) (end (line 41) (character 232))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemB") (range (start (line 41) (character 30)) (end (line 41) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (kind "part") (name "subsystem3") (declared-name "subsystem3") (range (start (line 42) (character 11)) (end (line 42) (character 92))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemB") (range (start (line 42) (character 29)) (end (line 42) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind "part") (name "part4") (range (start (line 43) (character 4)) (end (line 43) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part4") (range (start (line 43) (character 13)) (end (line 43) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind "part") (name "part5") (range (start (line 44) (character 4)) (end (line 44) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 44) (character 13)) (end (line 44) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (kind "part") (name "subsystem4") (declared-name "subsystem4") (range (start (line 46) (character 11)) (end (line 46) (character 92))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemB") (range (start (line 46) (character 29)) (end (line 46) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind "part") (name "part5") (range (start (line 47) (character 4)) (end (line 47) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 47) (character 13)) (end (line 47) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind "part") (name "part6") (range (start (line 48) (character 4)) (end (line 48) (character 22))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part6") (range (start (line 48) (character 13)) (end (line 48) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::doX"))) (kind "action") (name "doX") (declared-name "doX") (range (start (line 2) (character 1)) (end (line 2) (character 12))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::doY"))) (kind "action") (name "doY") (declared-name "doY") (range (start (line 3) (character 1)) (end (line 3) (character 12))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part1"))) (kind "part") (name "part1") (declared-name "part1") (range (start (line 5) (character 1)) (end (line 5) (character 12))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part2"))) (kind "part") (name "part2") (declared-name "part2") (range (start (line 6) (character 1)) (end (line 6) (character 12))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))) (kind "part") (name "part3") (declared-name "part3") (range (start (line 7) (character 1)) (end (line 7) (character 27))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3::p1"))) (kind "port") (name "p1") (declared-name "p1") (range (start (line 8) (character 2)) (end (line 8) (character 10))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part4"))) (kind "part") (name "part4") (declared-name "part4") (range (start (line 10) (character 1)) (end (line 10) (character 12))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (kind "part") (name "part5") (declared-name "part5") (range (start (line 11) (character 1)) (end (line 11) (character 115))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (authored (membership (kind Feature)) (relationships (perform (reference "7a1-Variant Configuration - General Concept-a::part5::doXorY") (range none)))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY"))) (kind "action") (name "doXorY") (declared-name "doXorY") (range (start (line 13) (character 2)) (end (line 13) (character 87))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::p2"))) (kind "port") (name "p2") (declared-name "p2") (range (start (line 12) (character 2)) (end (line 12) (character 10))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part6"))) (kind "part") (name "part6") (declared-name "part6") (range (start (line 18) (character 1)) (end (line 18) (character 12))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind "part") (name "vehicleConfigA") (declared-name "vehicleConfigA") (range (start (line 61) (character 1)) (end (line 61) (character 210))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig") (range (start (line 61) (character 24)) (end (line 61) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (kind "part") (name "subsystemA") (range (start (line 62) (character 2)) (end (line 62) (character 47))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA") (range (start (line 62) (character 11)) (end (line 62) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (kind "part") (name "subsystemB") (range (start (line 63) (character 2)) (end (line 63) (character 114))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB") (range (start (line 63) (character 11)) (end (line 63) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind "part") (name "part5") (range (start (line 64) (character 3)) (end (line 64) (character 61))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 64) (character 12)) (end (line 64) (character 17)))) (perform (reference "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::") (range none)))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::"))) (kind "action") (name "") (range (start (line 65) (character 4)) (end (line 65) (character 36))) (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (kind redefinition) (ordinal 0)) (authored-target "part3") (range (start (line 21) (character 20)) (end (line 21) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 25) (character 20)) (end (line 25) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (range (start (line 70) (character 24)) (end (line 70) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (range (start (line 71) (character 11)) (end (line 71) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (range (start (line 72) (character 11)) (end (line 72) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 73) (character 12)) (end (line 73) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind performSource) (ordinal 0)) (authored-target "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind connectionSource) (ordinal 0)) (authored-target "subsystemA::part3::p1") (range (start (line 52) (character 16)) (end (line 52) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind connectionTarget) (ordinal 0)) (authored-target "subsystemB::part5::p2") (range (start (line 52) (character 44)) (end (line 52) (character 63))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemA") (range (start (line 30) (character 30)) (end (line 30) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemA") (range (start (line 31) (character 29)) (end (line 31) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind redefinition) (ordinal 0)) (authored-target "part1") (range (start (line 32) (character 13)) (end (line 32) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (range (start (line 33) (character 13)) (end (line 33) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemA") (range (start (line 35) (character 29)) (end (line 35) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (range (start (line 36) (character 13)) (end (line 36) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind redefinition) (ordinal 0)) (authored-target "part3") (range (start (line 37) (character 13)) (end (line 37) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemB") (range (start (line 41) (character 30)) (end (line 41) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemB") (range (start (line 42) (character 29)) (end (line 42) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind redefinition) (ordinal 0)) (authored-target "part4") (range (start (line 43) (character 13)) (end (line 43) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 44) (character 13)) (end (line 44) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemB") (range (start (line 46) (character 29)) (end (line 46) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 47) (character 13)) (end (line 47) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind redefinition) (ordinal 0)) (authored-target "part6") (range (start (line 48) (character 13)) (end (line 48) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (kind performSource) (ordinal 0)) (authored-target "7a1-Variant Configuration - General Concept-a::part5::doXorY") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (range (start (line 61) (character 24)) (end (line 61) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (range (start (line 62) (character 11)) (end (line 62) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (range (start (line 63) (character 11)) (end (line 63) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 64) (character 12)) (end (line 64) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind performSource) (ordinal 0)) (authored-target "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind redefinition) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (kind performSource) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind performSource) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
