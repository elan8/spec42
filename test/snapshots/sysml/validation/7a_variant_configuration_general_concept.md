# META
~~~ini
description=SysML Validation (07-Variant Configuration): 7a-Variant Configuration - General Concept
type=file
~~~
# SOURCE
~~~sysml
package '7a-Variant Configuration - General Concept' {
	
	part def Vehicle;
	
	part part1;
	part part2;
	part part3;
	part part4;
	part part5;
	part part6;
	
	abstract part anyVehicleConfig : Vehicle {
		
		variation part subsystemA {
			variant part subsystem1 {
				part :>> part1;
				part :>> part2;
			}
			variant part subsystem2 {
				part :>> part2;
				part :>> part3;
			}
		}

		variation part subsystemB {
			variant part subsystem3 {
				part :>> part4;
				part :>> part5;
			}
			variant part subsystem4 {
				part :>> part5;
				part :>> part6;
			}
		}
		
		assert constraint {
			subsystemA != subsystemA::subsystem2 | 
			subsystemB == subsystemB::subsystem3
		}
		
	}
	
	part vehicleConfigA :> anyVehicleConfig {		
		part :>> subsystemA = subsystemA::subsystem1;
		part :>> subsystemB = subsystemB::subsystem3;
	}
	
	part VehicleConfigB :> anyVehicleConfig {
		part :>> subsystemA = subsystemA::subsystem2;
		part :>> subsystemB = subsystemB::subsystem3;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "7a_variant_configuration_general_concept.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 4 1) (end 4 12))
      )
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
        (range (start 7 1) (end 7 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 8 1) (end 8 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 9 1) (end 9 12))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package '7a-Variant Configuration - General Concept' {

    part def Vehicle;

    part part1;
    part part2;
    part part3;
    part part4;
    part part5;
    part part6;

    abstract part anyVehicleConfig : Vehicle {

        variation part subsystemA {
            variant part subsystem1 {
                part :>> part1;
                part :>> part2;
            }
            variant part subsystem2 {
                part :>> part2;
                part :>> part3;
            }
        }

        variation part subsystemB {
            variant part subsystem3 {
                part :>> part4;
                part :>> part5;
            }
            variant part subsystem4 {
                part :>> part5;
                part :>> part6;
            }
        }

        assert constraint {
            subsystemA != subsystemA::subsystem2 |
            subsystemB == subsystemB::subsystem3
        }

    }

    part vehicleConfigA :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem1;
        part :>> subsystemB = subsystemB::subsystem3;
    }

    part VehicleConfigB :> anyVehicleConfig {
        part :>> subsystemA = subsystemA::subsystem2;
        part :>> subsystemB = subsystemB::subsystem3;
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5c20453adb32ef3923bdcfaadca1aba39d6ede6fc606fec40952b10dcd284608") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (kind "package") (name "7a-Variant Configuration - General Concept") (declared-name "7a-Variant Configuration - General Concept") (range (start (line 0) (character 0)) (end (line 0) (character 981))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 2) (character 1)) (end (line 2) (character 18))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind "part") (name "VehicleConfigB") (declared-name "VehicleConfigB") (range (start (line 47) (character 1)) (end (line 47) (character 141))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig") (range (start (line 47) (character 24)) (end (line 47) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (kind "part") (name "subsystemA") (range (start (line 48) (character 2)) (end (line 48) (character 47))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA") (range (start (line 48) (character 11)) (end (line 48) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (kind "part") (name "subsystemB") (range (start (line 49) (character 2)) (end (line 49) (character 47))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB") (range (start (line 49) (character 11)) (end (line 49) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind "part") (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (range (start (line 11) (character 1)) (end (line 11) (character 529))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 11) (character 34)) (end (line 11) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))) (kind "part") (name "subsystemA") (declared-name "subsystemA") (range (start (line 13) (character 2)) (end (line 13) (character 181))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (kind "part") (name "subsystem1") (declared-name "subsystem1") (range (start (line 14) (character 11)) (end (line 14) (character 73))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind "part") (name "part1") (range (start (line 15) (character 4)) (end (line 15) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part1") (range (start (line 15) (character 13)) (end (line 15) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind "part") (name "part2") (range (start (line 16) (character 4)) (end (line 16) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2") (range (start (line 16) (character 13)) (end (line 16) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (kind "part") (name "subsystem2") (declared-name "subsystem2") (range (start (line 18) (character 11)) (end (line 18) (character 73))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind "part") (name "part2") (range (start (line 19) (character 4)) (end (line 19) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2") (range (start (line 19) (character 13)) (end (line 19) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind "part") (name "part3") (range (start (line 20) (character 4)) (end (line 20) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part3") (range (start (line 20) (character 13)) (end (line 20) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))) (kind "part") (name "subsystemB") (declared-name "subsystemB") (range (start (line 24) (character 2)) (end (line 24) (character 181))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (kind "part") (name "subsystem3") (declared-name "subsystem3") (range (start (line 25) (character 11)) (end (line 25) (character 73))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind "part") (name "part4") (range (start (line 26) (character 4)) (end (line 26) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part4") (range (start (line 26) (character 13)) (end (line 26) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind "part") (name "part5") (range (start (line 27) (character 4)) (end (line 27) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 27) (character 13)) (end (line 27) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (kind "part") (name "subsystem4") (declared-name "subsystem4") (range (start (line 29) (character 11)) (end (line 29) (character 73))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind "part") (name "part5") (range (start (line 30) (character 4)) (end (line 30) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5") (range (start (line 30) (character 13)) (end (line 30) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind "part") (name "part6") (range (start (line 31) (character 4)) (end (line 31) (character 19))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part6") (range (start (line 31) (character 13)) (end (line 31) (character 18)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part1"))) (kind "part") (name "part1") (declared-name "part1") (range (start (line 4) (character 1)) (end (line 4) (character 12))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part2"))) (kind "part") (name "part2") (declared-name "part2") (range (start (line 5) (character 1)) (end (line 5) (character 12))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part3"))) (kind "part") (name "part3") (declared-name "part3") (range (start (line 6) (character 1)) (end (line 6) (character 12))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part4"))) (kind "part") (name "part4") (declared-name "part4") (range (start (line 7) (character 1)) (end (line 7) (character 12))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part5"))) (kind "part") (name "part5") (declared-name "part5") (range (start (line 8) (character 1)) (end (line 8) (character 12))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part6"))) (kind "part") (name "part6") (declared-name "part6") (range (start (line 9) (character 1)) (end (line 9) (character 12))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind "part") (name "vehicleConfigA") (declared-name "vehicleConfigA") (range (start (line 42) (character 1)) (end (line 42) (character 143))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig") (range (start (line 42) (character 24)) (end (line 42) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (kind "part") (name "subsystemA") (range (start (line 43) (character 2)) (end (line 43) (character 47))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA") (range (start (line 43) (character 11)) (end (line 43) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (kind "part") (name "subsystemB") (range (start (line 44) (character 2)) (end (line 44) (character 47))) (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB") (range (start (line 44) (character 11)) (end (line 44) (character 21)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (range (start (line 47) (character 24)) (end (line 47) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (range (start (line 48) (character 11)) (end (line 48) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (range (start (line 49) (character 11)) (end (line 49) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 11) (character 34)) (end (line 11) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind redefinition) (ordinal 0)) (authored-target "part1") (range (start (line 15) (character 13)) (end (line 15) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (range (start (line 16) (character 13)) (end (line 16) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (range (start (line 19) (character 13)) (end (line 19) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind redefinition) (ordinal 0)) (authored-target "part3") (range (start (line 20) (character 13)) (end (line 20) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind redefinition) (ordinal 0)) (authored-target "part4") (range (start (line 26) (character 13)) (end (line 26) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 27) (character 13)) (end (line 27) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (range (start (line 30) (character 13)) (end (line 30) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind redefinition) (ordinal 0)) (authored-target "part6") (range (start (line 31) (character 13)) (end (line 31) (character 18))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (range (start (line 42) (character 24)) (end (line 42) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (range (start (line 43) (character 11)) (end (line 43) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (range (start (line 44) (character 11)) (end (line 44) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB")))))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 15 13) (end 15 18)) (probe (position 15 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))
        (kind redefinition) (ordinal 0) (authored-target "part1")
        (range (start 15 13) (end 15 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1") (range (start 15 4) (end 15 19)))
        )
      )
    )
    (query (range (start 16 13) (end 16 18)) (probe (position 16 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))
        (kind redefinition) (ordinal 0) (authored-target "part2")
        (range (start 16 13) (end 16 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2") (range (start 16 4) (end 16 19)))
        )
      )
    )
    (query (range (start 19 13) (end 19 18)) (probe (position 19 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))
        (kind redefinition) (ordinal 0) (authored-target "part2")
        (range (start 19 13) (end 19 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2") (range (start 19 4) (end 19 19)))
        )
      )
    )
    (query (range (start 20 13) (end 20 18)) (probe (position 20 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))
        (kind redefinition) (ordinal 0) (authored-target "part3")
        (range (start 20 13) (end 20 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3") (range (start 20 4) (end 20 19)))
        )
      )
    )
    (query (range (start 26 13) (end 26 18)) (probe (position 26 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))
        (kind redefinition) (ordinal 0) (authored-target "part4")
        (range (start 26 13) (end 26 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4") (range (start 26 4) (end 26 19)))
        )
      )
    )
    (query (range (start 27 13) (end 27 18)) (probe (position 27 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 27 13) (end 27 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5") (range (start 27 4) (end 27 19)))
        )
      )
    )
    (query (range (start 30 13) (end 30 18)) (probe (position 30 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))
        (kind redefinition) (ordinal 0) (authored-target "part5")
        (range (start 30 13) (end 30 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5") (range (start 30 4) (end 30 19)))
        )
      )
    )
    (query (range (start 31 13) (end 31 18)) (probe (position 31 13))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))
        (kind redefinition) (ordinal 0) (authored-target "part6")
        (range (start 31 13) (end 31 18))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6") (range (start 31 4) (end 31 19)))
        )
      )
    )
    (query (range (start 11 34) (end 11 41)) (probe (position 11 34))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 11 34) (end 11 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle") (range (start 2 1) (end 2 18)))
        )
      )
    )
    (query (range (start 43 11) (end 43 21)) (probe (position 43 11))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemA")
        (range (start 43 11) (end 43 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA") (range (start 43 2) (end 43 47)))
        )
      )
    )
    (query (range (start 44 11) (end 44 21)) (probe (position 44 11))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemB")
        (range (start 44 11) (end 44 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB") (range (start 44 2) (end 44 47)))
        )
      )
    )
    (query (range (start 48 11) (end 48 21)) (probe (position 48 11))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemA")
        (range (start 48 11) (end 48 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA") (range (start 48 2) (end 48 47)))
        )
      )
    )
    (query (range (start 49 11) (end 49 21)) (probe (position 49 11))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))
        (kind redefinition) (ordinal 0) (authored-target "subsystemB")
        (range (start 49 11) (end 49 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB") (range (start 49 2) (end 49 47)))
        )
      )
    )
    (query (range (start 42 24) (end 42 40)) (probe (position 42 24))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))
        (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
        (range (start 42 24) (end 42 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig") (range (start 11 1) (end 11 529)))
        )
      )
    )
    (query (range (start 47 24) (end 47 40)) (probe (position 47 24))
      (reference
        (source (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))
        (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
        (range (start 47 24) (end 47 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig") (range (start 11 1) (end 11 529)))
        )
      )
    )
  )
)
~~~
