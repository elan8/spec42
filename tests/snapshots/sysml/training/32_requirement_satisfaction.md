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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 44))
      )
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
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 17 1) (end 24 2))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 10) (end 21 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 10) (end 22 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 33) (end 22 57))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:1e8a2b7305e34b8aaaf63fa42a5a05e72b9be46cd4df689a44c6ae6762ecf101"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Groups") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "vehicleSpecification")) (satisfyTarget (reference "vehicle_design")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "engineSpecification")) (memberAccessOperand (reference "vehicle_design::engine_v1")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (kind ref) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "vehicle_c1")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (kind action) (membership (kind feature) (visibility default)) (facts (modifiers composite)))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (referenceSubsetting (reference "provide power")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind port) (ordinal 0))))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "clutchPort")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "generateTorque")) (referenceSubsetting (reference "provide power::generate torque")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Groups")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "vehicleSpecification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0))
      (authored-target "engineSpecification")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "vehicle_design")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle_design::engine_v1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "provide power")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "clutchPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "generateTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0))
      (authored-target "provide power::generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque")))))
  )
  (relationships
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (kind subsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque"))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind port) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1)))))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design")))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context")))
      (supertype (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque")))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1")))
      (subtype (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1")))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind port) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 2 16) (end 2 39)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Groups")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 21 10) (end 21 30)) (probe (position 21 10))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "vehicleSpecification")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 22 10) (end 22 29)) (probe (position 22 10))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1))))) (kind satisfySource) (ordinal 0) (authored-target "engineSpecification")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 21 34) (end 21 48)) (probe (position 21 34))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0) (authored-target "vehicle_design")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 22 33) (end 22 57)) (probe (position 22 33))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "Vehicle c1 Design Context")) (anonymous (kind satisfy) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle_design::engine_v1")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 19 24) (end 19 34)) (probe (position 19 24))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::Vehicle c1 Design Context::vehicle_design"))) (kind subsetting) (ordinal 0) (authored-target "vehicle_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 8 19) (end 8 26)) (probe (position 8 19))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 9 10) (end 9 25)) (probe (position 9 10))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "provide power")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 11 18) (end 11 24)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::vehicle_c1::engine_v1"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 12 12) (end 12 22)) (probe (position 12 12))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind port) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "clutchPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 13 49) (end 13 63)) (probe (position 13 49))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "generateTorque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_satisfaction.md") (range (start 13 11) (end 13 44)) (probe (position 13 11))
    (reference (id (source (node (document "memory://snapshot/32_requirement_satisfaction.md") (path (named (kind package) (name "Requirement Satisfaction")) (named (kind part) (name "vehicle_c1")) (named (kind part) (name "engine_v1")) (anonymous (kind perform-action) (ordinal 0))))) (kind referenceSubsetting) (ordinal 0) (authored-target "provide power::generate torque")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_satisfaction.md") (qualified-name "Requirement Satisfaction::provide power::generate torque")))))
    )
  )
)
~~~
