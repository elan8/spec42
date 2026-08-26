# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Groups
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Groups' {
	private import 'Requirement Definitions'::*;
	private import 'Requirement Usages'::*;
	
	part def Engine {
		port clutchPort: ClutchPort;
		perform action generateTorque: GenerateTorque;
	}
	
	requirement vehicleSpecification {
		doc /* Overall vehicle requirements group */
		
		subject vehicle : Vehicle;
		
		require fullVehicleMassLimit;
		require emptyVehicleMassLimit;
	}
	
	requirement engineSpecification {
		doc /* Engine power requirements group */
		
		subject engine : Engine;
		
		requirement drivePowerInterface : DrivePowerInterface {
			subject = engine.clutchPort;
		}
		
		requirement torqueGeneration : TorqueGeneration {
			subject = engine.generateTorque;	
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/32_requirement_groups.md"
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
        (severity information)
        (code "unconnected_port")
        (source "semantic")
        (range (start 5 2) (end 5 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 19) (end 5 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 33) (end 6 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 20) (end 12 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 14 2) (end 14 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 36) (end 23 55))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 33) (end 27 49))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:58e56b8fd4ce5c3a81dfa62cea756099f30af3890be9075d2905e655b81da8e3") (contract-version "operator-expression-arguments-v7"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Usages") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind port) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ClutchPort")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind perform-action) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "GenerateTorque")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Engine power requirements group "))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DrivePowerInterface")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TorqueGeneration")))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " Overall vehicle requirements group "))))
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Usages")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind featureTyping) (ordinal 0))
      (authored-target "ClutchPort")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind featureTyping) (ordinal 0))
      (authored-target "GenerateTorque")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind featureTyping) (ordinal 0))
      (authored-target "DrivePowerInterface")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")))))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind featureTyping) (ordinal 0))
      (authored-target "TorqueGeneration")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::clutchPort"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::generateTorque"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification"))) (provenance implied))
  )
  (evaluation
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")))
      (subtype (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::clutchPort")))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::generateTorque")))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface")))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface")))
      (supertype (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "drivePowerInterface")) (anonymous (kind subject) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine")))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification")))
      (type (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")) (provenance authored))
      (effective-type (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")) (source direct))
      (supertype (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration")))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification")))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration")))
      (supertype (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (named (kind requirement) (name "engineSpecification")) (named (kind requirement) (name "torqueGeneration")) (anonymous (kind subject) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification::vehicle")))
      (featured-by (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 1 16) (end 1 44)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 2 16) (end 2 39)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (path (named (kind package) (name "Requirement Groups")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Usages")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 5 19) (end 5 29)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::clutchPort"))) (kind featureTyping) (ordinal 0) (authored-target "ClutchPort")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 6 33) (end 6 47)) (probe (position 6 33))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine::generateTorque"))) (kind featureTyping) (ordinal 0) (authored-target "GenerateTorque")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 23 36) (end 23 55)) (probe (position 23 36))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::drivePowerInterface"))) (kind featureTyping) (ordinal 0) (authored-target "DrivePowerInterface")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 21 19) (end 21 25)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::Engine")))))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 27 33) (end 27 49)) (probe (position 27 33))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::engineSpecification::torqueGeneration"))) (kind featureTyping) (ordinal 0) (authored-target "TorqueGeneration")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/32_requirement_groups.md") (range (start 12 20) (end 12 27)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/32_requirement_groups.md") (qualified-name "Requirement Groups::vehicleSpecification::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
)
~~~
