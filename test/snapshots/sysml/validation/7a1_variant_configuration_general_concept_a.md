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
  (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 31 3) (end 34 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 35 3) (end 38 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 42 3) (end 45 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 46 3) (end 49 4))
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
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 55 17) (end 56 13))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 56 17) (end 56 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 62 11) (end 62 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 63 11) (end 63 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 65 23) (end 65 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 71 11) (end 71 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 72 11) (end 72 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 74 23) (end 74 29))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:c71c2b583b4093685158872ef283ed62db7a7323892011b8ddf86da12b503f13") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "part3"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "part5"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "anyVehicleConfig"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemA"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemB"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "part5"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "doXorY"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind bare-connect) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "subsystemA::part3::p1")) (memberAccessOperand (reference "subsystemB::part5::p2"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "subsystemA")) (expressionOperand (reference "subsystemB::subsystem3"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubsystemA") (variation true))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SubsystemB") (variation true))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::doX"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::doY"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part3::p1"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part4"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::doXorY"))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 1))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5::p2"))) (kind port) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part6"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "anyVehicleConfig"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemA"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemB"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "part5"))))
    (declaration (id (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "doXorY"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "part3")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part3")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "part5")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind subsetting) (ordinal 0))
      (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemA")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemB")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "part5")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "doXorY")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "subsystemA")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsystemB::subsystem3")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "subsystemA::part3::p1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "subsystemB::part5::p2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubsystemA")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind featureTyping) (ordinal 0))
      (authored-target "SubsystemB")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind subsetting) (ordinal 0))
      (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemA")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemB")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "part5")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5")))))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "doXorY")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part3"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 21 20) (end 21 25)) (probe (position 21 20))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "part3")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part3")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 25 20) (end 25 25)) (probe (position 25 20))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "part5")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 70 24) (end 70 40)) (probe (position 70 24))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::VehicleConfigB"))) (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 71 11) (end 71 21)) (probe (position 71 11))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subsystemA")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 72 11) (end 72 21)) (probe (position 72 11))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "subsystemB")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 73 12) (end 73 17)) (probe (position 73 12))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "part5")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 74 23) (end 74 29)) (probe (position 74 23))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "doXorY")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 55 3) (end 55 13)) (probe (position 55 3))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "subsystemA")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 56 17) (end 56 39)) (probe (position 56 17))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "subsystemB::subsystem3")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 52 16) (end 52 36)) (probe (position 52 16))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "subsystemA::part3::p1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 52 44) (end 52 63)) (probe (position 52 44))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind bare-connect) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "subsystemB::part5::p2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 30 30) (end 30 40)) (probe (position 30 30))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemA"))) (kind featureTyping) (ordinal 0) (authored-target "SubsystemA")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemA")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 41 30) (end 41 40)) (probe (position 41 30))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig::subsystemB"))) (kind featureTyping) (ordinal 0) (authored-target "SubsystemB")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::SubsystemB")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 61 24) (end 61 40)) (probe (position 61 24))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::vehicleConfigA"))) (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::anyVehicleConfig")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 62 11) (end 62 21)) (probe (position 62 11))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subsystemA")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 63 11) (end 63 21)) (probe (position 63 11))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "subsystemB")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 64 12) (end 64 17)) (probe (position 64 12))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "part5")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (qualified-name "7a1-Variant Configuration - General Concept-a::part5")))))
  )
  (query (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (range (start 65 23) (end 65 29)) (probe (position 65 23))
    (reference (id (source (node (document "memory://snapshot/7a1_variant_configuration_general_concept_a.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "doXorY")
      (outcome (status unresolved)))
  )
)
~~~
