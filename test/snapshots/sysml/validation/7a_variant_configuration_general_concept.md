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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5c20453adb32ef3923bdcfaadca1aba39d6ede6fc606fec40952b10dcd284608") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (kind "package") (name "7a-Variant Configuration - General Concept") (declared-name "7a-Variant Configuration - General Concept"))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind "part") (name "VehicleConfigB") (declared-name "VehicleConfigB") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (kind "part") (name "subsystemA") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (kind "part") (name "subsystemB") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind "part") (name "anyVehicleConfig") (declared-name "anyVehicleConfig") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))) (kind "part") (name "subsystemA") (declared-name "subsystemA") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (kind "part") (name "subsystem1") (declared-name "subsystem1") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind "part") (name "part1") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part1")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind "part") (name "part2") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (kind "part") (name "subsystem2") (declared-name "subsystem2") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind "part") (name "part2") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part2")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind "part") (name "part3") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part3")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))) (kind "part") (name "subsystemB") (declared-name "subsystemB") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (kind "part") (name "subsystem3") (declared-name "subsystem3") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind "part") (name "part4") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part4")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (kind "part") (name "subsystem4") (declared-name "subsystem4") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind "part") (name "part5") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part5")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind "part") (name "part6") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "part6")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part1"))) (kind "part") (name "part1") (declared-name "part1") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part2"))) (kind "part") (name "part2") (declared-name "part2") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part3"))) (kind "part") (name "part3") (declared-name "part3") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part4"))) (kind "part") (name "part4") (declared-name "part4") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part5"))) (kind "part") (name "part5") (declared-name "part5") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::part6"))) (kind "part") (name "part6") (declared-name "part6") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind "part") (name "vehicleConfigA") (declared-name "vehicleConfigA") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (kind "part") (name "subsystemA") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemA")))))
    (element (id (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (kind "part") (name "subsystemB") (parent (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "subsystemB")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB::subsystemB")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1"))) (kind redefinition) (ordinal 0)) (authored-target "part1") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part1")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem1::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2"))) (kind redefinition) (ordinal 0)) (authored-target "part2") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part2")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3"))) (kind redefinition) (ordinal 0)) (authored-target "part3") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA::subsystem2::part3")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4"))) (kind redefinition) (ordinal 0)) (authored-target "part4") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part4")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem3::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5"))) (kind redefinition) (ordinal 0)) (authored-target "part5") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part5")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6"))) (kind redefinition) (ordinal 0)) (authored-target "part6") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB::subsystem4::part6")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind subsetting) (ordinal 0)) (authored-target "anyVehicleConfig") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemA") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemA")))))
    (reference (id (source (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB"))) (kind redefinition) (ordinal 0)) (authored-target "subsystemB") (outcome (status resolved) (target (node (document "d0") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA::subsystemB")))))
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
