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
  (document "memory://snapshot/7a_variant_configuration_general_concept.md"
    (diagnostics
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 4 1) (end 4 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 1) (end 5 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 1) (end 6 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 7 1) (end 7 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 8 1) (end 8 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 9 1) (end 9 12))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 13 2) (end 22 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 14 3) (end 17 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 18 3) (end 21 4))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 24 2) (end 33 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 25 3) (end 28 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 29 3) (end 32 4))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 35 2) (end 38 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 36 17) (end 36 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 17) (end 37 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 11) (end 43 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 44 11) (end 44 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 48 11) (end 48 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 49 11) (end 49 21))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:c612a9f55e435f84c0eb2e66ae18f1c8ec4581460065123c2c30ee0a7d47859e") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemA")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemB")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "subsystemA")) (expressionOperand (reference "subsystemA::subsystem2")) (expressionOperand (reference "subsystemB")) (expressionOperand (reference "subsystemB::subsystem3")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::part1"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::part2"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::part3"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::part4"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::part5"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::part6"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "anyVehicleConfig")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemA")))))
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 1))))) (kind part) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "subsystemB")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind subsetting) (ordinal 0))
      (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemA")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemB")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "subsystemA")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA")))))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "subsystemA::subsystem2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "subsystemB")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB")))))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 3))
      (authored-target "subsystemB::subsystem3")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind subsetting) (ordinal 0))
      (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemA")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "subsystemB")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind subsetting) (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))
      (subtype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB")))
      (effective-type (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (source inherited) (from (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))))
      (supertype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB")))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB")))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))
      (type (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB")) (scopes any feature))
      (subtype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA")))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB")))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA")))
      (effective-type (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (source inherited) (from (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))))
      (supertype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")) (scopes any))
      (supertype (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA")))
    )
    (declaration (id (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 47 24) (end 47 40)) (probe (position 47 24))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::VehicleConfigB"))) (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 48 11) (end 48 21)) (probe (position 48 11))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subsystemA")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 49 11) (end 49 21)) (probe (position 49 11))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "VehicleConfigB")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "subsystemB")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 11 34) (end 11 41)) (probe (position 11 34))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 36 3) (end 36 13)) (probe (position 36 3))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "subsystemA")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemA")))))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 36 17) (end 36 39)) (probe (position 36 17))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "subsystemA::subsystem2")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 37 3) (end 37 13)) (probe (position 37 3))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "subsystemB")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig::subsystemB")))))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 37 17) (end 37 39)) (probe (position 37 17))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "anyVehicleConfig")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 3) (authored-target "subsystemB::subsystem3")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 42 24) (end 42 40)) (probe (position 42 24))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::vehicleConfigA"))) (kind subsetting) (ordinal 0) (authored-target "anyVehicleConfig")
      (outcome (status resolved) (target (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (qualified-name "7a-Variant Configuration - General Concept::anyVehicleConfig")))))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 43 11) (end 43 21)) (probe (position 43 11))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "subsystemA")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/7a_variant_configuration_general_concept.md") (range (start 44 11) (end 44 21)) (probe (position 44 11))
    (reference (id (source (node (document "memory://snapshot/7a_variant_configuration_general_concept.md") (path (named (kind package) (name "7a-Variant Configuration - General Concept")) (named (kind part) (name "vehicleConfigA")) (anonymous (kind part) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "subsystemB")
      (outcome (status unresolved)))
    )
  )
)
~~~
