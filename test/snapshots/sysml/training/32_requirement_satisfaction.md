# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Satisfaction
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Satisfaction' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Groups'::*;
	
	action 'provide power' {
		action 'generate torque' { }
	}
	
	part vehicle_c1 : Vehicle {
		perform 'provide power';
			
		part engine_v1: Engine {
			port :>> clutchPort;
			perform 'provide power'.'generate torque' :>> generateTorque;
		}	
	}
	
	part 'Vehicle c1 Design Context' {
		
		ref vehicle_design :> vehicle_c1;
	
		satisfy vehicleSpecification by vehicle_design;
		satisfy engineSpecification by vehicle_design.engine_v1;
	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/32_requirement_satisfaction.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 19) (end 8 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 12) (end 12 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 49) (end 13 63))
      )
      (diagnostic
        (severity error)
        (code "recovered_part_usage_body_element")
        (source "parser")
        (range (start 19 2) (end 21 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 21 2) (end 21 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 22 2) (end 22 58))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:1e8a2b7305e34b8aaaf63fa42a5a05e72b9be46cd4df689a44c6ae6762ecf101") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Groups") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "clutchPort"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "generateTorque"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Groups")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "clutchPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "generateTorque")
      (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 2 16) (end 2 39)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Groups")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 8 19) (end 8 26)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 11 18) (end 11 24)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 12 12) (end 12 22)) (probe (position 12 12))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "clutchPort")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 13 49) (end 13 63)) (probe (position 13 49))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "generateTorque")
      (outcome (status unresolved)))
  )
)
~~~
