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
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (kind "package") (name "7a1-Variant Configuration - General Concept-a") (declared-name "7a1-Variant Configuration - General Concept-a"))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (kind "part def") (name "SubsystemA") (declared-name "SubsystemA") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (kind "part") (name "part3") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part3")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (kind "part def") (name "SubsystemB") (declared-name "SubsystemB") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind "part") (name "VehicleConfigB") (declared-name "VehicleConfigB") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (kind "part") (name "subsystemA") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (kind "part") (name "subsystemB") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")) (perform (reference "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind "part") (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind "part") (name "subsystemA") (declared-name "subsystemA") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (kind "part") (name "subsystem1") (declared-name "subsystem1") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind "part") (name "part1") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part1")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind "part") (name "part2") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (kind "part") (name "subsystem2") (declared-name "subsystem2") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind "part") (name "part2") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind "part") (name "part3") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part3")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind "part") (name "subsystemB") (declared-name "subsystemB") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemB")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (kind "part") (name "subsystem3") (declared-name "subsystem3") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemB")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind "part") (name "part4") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part4")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (kind "part") (name "subsystem4") (declared-name "subsystem4") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (authored (membership (kind Feature)) (relationships (typing (reference "SubsystemB")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind "part") (name "part6") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part6")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::doX"))) (kind "action") (name "doX") (declared-name "doX") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::doY"))) (kind "action") (name "doY") (declared-name "doY") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part1"))) (kind "part") (name "part1") (declared-name "part1") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part2"))) (kind "part") (name "part2") (declared-name "part2") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))) (kind "part") (name "part3") (declared-name "part3") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3::p1"))) (kind "port") (name "p1") (declared-name "p1") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part4"))) (kind "part") (name "part4") (declared-name "part4") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (kind "part") (name "part5") (declared-name "part5") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (authored (membership (kind Feature)) (relationships (perform (reference "7a1-Variant Configuration - General Concept-a::part5::doXorY")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY"))) (kind "action") (name "doXorY") (declared-name "doXorY") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::p2"))) (kind "port") (name "p2") (declared-name "p2") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part6"))) (kind "part") (name "part6") (declared-name "part6") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind "part") (name "vehicleConfigA") (declared-name "vehicleConfigA") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (kind "part") (name "subsystemA") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (kind "part") (name "subsystemB") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")) (perform (reference "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::")))))
    (element (id (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::"))) (kind "action") (name "") (parent (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))) (kind redefinition) (ordinal 0)) (authored-target "part3") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))) (kind performSource) (ordinal 0)) (authored-target "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5::")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind connectionSource) (ordinal 0)) (authored-target "subsystemA::part3::p1") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind connectionTarget) (ordinal 0)) (authored-target "subsystemB::part5::p2") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind redefinition) (ordinal 0)) (authored-target "part1") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind redefinition) (ordinal 0)) (authored-target "part3") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind redefinition) (ordinal 0)) (authored-target "part4") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))) (kind featureTyping) (ordinal 0)) (authored-target "SubsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind redefinition) (ordinal 0)) (authored-target "part6") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (kind performSource) (ordinal 0)) (authored-target "7a1-Variant Configuration - General Concept-a::part5::doXorY") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))) (kind performSource) (ordinal 0)) (authored-target "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5::")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 21 20) (end 21 25)) (probe (position 21 20))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3"))
        (kind redefinition) (ordinal 0) (authored-target "part3")
        (range (start 21 20) (end 21 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA::part3") (range (start 21 2) (end 21 32)))
        )
      )
    )
    (query (range (start 25 20) (end 25 25)) (probe (position 25 20))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 25 20) (end 25 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB::part5") (range (start 25 2) (end 25 29)))
        )
      )
    )
    (query (range (start 32 13) (end 32 18)) (probe (position 32 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1"))
        (kind redefinition) (ordinal 0) (authored-target "part1")
        (range (start 32 13) (end 32 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part1") (range (start 32 4) (end 32 22)))
        )
      )
    )
    (query (range (start 33 13) (end 33 18)) (probe (position 33 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2"))
        (kind redefinition) (ordinal 0) (authored-target "part2")
        (range (start 33 13) (end 33 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1::part2") (range (start 33 4) (end 33 22)))
        )
      )
    )
    (query (range (start 36 13) (end 36 18)) (probe (position 36 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2"))
        (kind redefinition) (ordinal 0) (authored-target "part2")
        (range (start 36 13) (end 36 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part2") (range (start 36 4) (end 36 22)))
        )
      )
    )
    (query (range (start 37 13) (end 37 18)) (probe (position 37 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3"))
        (kind redefinition) (ordinal 0) (authored-target "part3")
        (range (start 37 13) (end 37 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2::part3") (range (start 37 4) (end 37 22)))
        )
      )
    )
    (query (range (start 43 13) (end 43 18)) (probe (position 43 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4"))
        (kind redefinition) (ordinal 0) (authored-target "part4")
        (range (start 43 13) (end 43 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part4") (range (start 43 4) (end 43 22)))
        )
      )
    )
    (query (range (start 44 13) (end 44 18)) (probe (position 44 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 44 13) (end 44 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3::part5") (range (start 44 4) (end 44 22)))
        )
      )
    )
    (query (range (start 47 13) (end 47 18)) (probe (position 47 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 47 13) (end 47 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part5") (range (start 47 4) (end 47 22)))
        )
      )
    )
    (query (range (start 48 13) (end 48 18)) (probe (position 48 13))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6"))
        (kind redefinition) (ordinal 0) (authored-target "part6")
        (range (start 48 13) (end 48 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4::part6") (range (start 48 4) (end 48 22)))
        )
      )
    )
    (query (range (start 64 12) (end 64 17)) (probe (position 64 12))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 64 12) (end 64 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB::part5") (range (start 64 3) (end 64 61)))
        )
      )
    )
    (query (range (start 73 12) (end 73 17)) (probe (position 73 12))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 73 12) (end 73 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB::part5") (range (start 73 3) (end 73 61)))
        )
      )
    )
    (query (range (start 30 30) (end 30 40)) (probe (position 30 30))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))
        (kind featureTyping) (ordinal 0) (authored-target "SubsystemA")
        (range (start 30 30) (end 30 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA") (range (start 20 1) (end 20 67)))
        )
      )
    )
    (query (range (start 31 29) (end 31 39)) (probe (position 31 29))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem1"))
        (kind featureTyping) (ordinal 0) (authored-target "SubsystemA")
        (range (start 31 29) (end 31 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA") (range (start 20 1) (end 20 67)))
        )
      )
    )
    (query (range (start 35 29) (end 35 39)) (probe (position 35 29))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA::subsystem2"))
        (kind featureTyping) (ordinal 0) (authored-target "SubsystemA")
        (range (start 35 29) (end 35 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA") (range (start 20 1) (end 20 67)))
        )
      )
    )
    (query (range (start 41 30) (end 41 40)) (probe (position 41 30))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))
        (kind featureTyping) (ordinal 0) (authored-target "SubsystemB")
        (range (start 41 30) (end 41 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB") (range (start 24 1) (end 24 66)))
        )
      )
    )
    (query (range (start 42 29) (end 42 39)) (probe (position 42 29))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem3"))
        (kind featureTyping) (ordinal 0) (authored-target "SubsystemB")
        (range (start 42 29) (end 42 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB") (range (start 24 1) (end 24 66)))
        )
      )
    )
    (query (range (start 46 29) (end 46 39)) (probe (position 46 29))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB::subsystem4"))
        (kind featureTyping) (ordinal 0) (authored-target "SubsystemB")
        (range (start 46 29) (end 46 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB") (range (start 24 1) (end 24 66)))
        )
      )
    )
    (query (range (start 62 11) (end 62 21)) (probe (position 62 11))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemA")
        (range (start 62 11) (end 62 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemA") (range (start 62 2) (end 62 47)))
        )
      )
    )
    (query (range (start 63 11) (end 63 21)) (probe (position 63 11))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemB")
        (range (start 63 11) (end 63 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA::subsystemB") (range (start 63 2) (end 63 114)))
        )
      )
    )
    (query (range (start 71 11) (end 71 21)) (probe (position 71 11))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemA")
        (range (start 71 11) (end 71 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemA") (range (start 71 2) (end 71 47)))
        )
      )
    )
    (query (range (start 72 11) (end 72 21)) (probe (position 72 11))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemB")
        (range (start 72 11) (end 72 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB::subsystemB") (range (start 72 2) (end 72 114)))
        )
      )
    )
    (query (range (start 61 24) (end 61 40)) (probe (position 61 24))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))
        (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
        (range (start 61 24) (end 61 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig") (range (start 28 1) (end 28 680)))
        )
      )
    )
    (query (range (start 70 24) (end 70 40)) (probe (position 70 24))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))
        (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
        (range (start 70 24) (end 70 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig") (range (start 28 1) (end 28 680)))
        )
      )
    )
    (query (range (start 52 44) (end 52 63)) (probe (position 52 44))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))
        (kind connectionTarget) (ordinal 0) (authored-target "subsystemB::part5::p2")
        (range (start 52 44) (end 52 63))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 16) (end 52 36)) (probe (position 52 16))
      (reference
        (source (document "d0") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))
        (kind connectionSource) (ordinal 0) (authored-target "subsystemA::part3::p1")
        (range (start 52 16) (end 52 36))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
