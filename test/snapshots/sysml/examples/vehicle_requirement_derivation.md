# META
~~~ini
description=SysML Example (Requirements): VehicleRequirementDerivation
type=file
~~~
# SOURCE
~~~sysml
package VehicleRequirementDerivation {
	private import RequirementDerivation::*;
	
	part vehicle {
		attribute mass :> ISQ::mass;
		
		part chassis {
			attribute mass :> ISQ::mass;
		}
		
		part engine {
			attribute mass :> ISQ::mass;
		}
	}
	
	requirement def MassRequirement {
		subject mass :> ISQ::mass;
		attribute massLimit :> ISQ::mass;
		require constraint { mass <= massLimit }
	}
	
	requirement vehicleMassRequirement : MassRequirement {
		subject :>> mass = vehicle.mass;
	}
	
	requirement chassisMassRequirement : MassRequirement {
		subject :>> mass = vehicle.chassis.mass;
	}
	
	requirement engineMassRequirement : MassRequirement {
		subject :>> mass = vehicle.engine.mass;
	}
	
	#derivation connection {
		end #original ::> vehicleMassRequirement;
		end #derive ::> chassisMassRequirement;
		end #derive ::> engineMassRequirement;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/vehicle_requirement_derivation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 4 20) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 21) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 21) (end 11 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 25) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 23) (end 18 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 22 2) (end 22 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 26 2) (end 26 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 30 2) (end 30 41))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:69413829c57e7c8016e64aecc70f3ed72bc64ca31c58aa323e9c50563e7f0493") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementDerivation") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection-def) (ordinal 0))))) (kind connection-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 0))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "vehicleMassRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 1))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "chassisMassRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 2))))) (kind connection) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (connectorEnd (reference "engineMassRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "mass")) (expressionOperand (reference "massLimit"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRequirement"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::chassis"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRequirement"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0))
      (authored-target "vehicleMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0))
      (authored-target "chassisMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 2))))) (kind connectorEnd) (ordinal 0))
      (authored-target "engineMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "massLimit")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
  (relationships
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 1))))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind connectorEnd) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 2))))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 2))))) (kind connectorEnd) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (value (kind unresolved-operand)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 1 16) (end 1 40)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "RequirementDerivation")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 34 20) (end 34 42)) (probe (position 34 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 0))))) (kind connectorEnd) (ordinal 0) (authored-target "vehicleMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 35 18) (end 35 40)) (probe (position 35 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 1))))) (kind connectorEnd) (ordinal 0) (authored-target "chassisMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 36 18) (end 36 39)) (probe (position 36 18))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind connection) (ordinal 2))))) (kind connectorEnd) (ordinal 0) (authored-target "engineMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 18 23) (end 18 27)) (probe (position 18 23))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 18 31) (end 18 40)) (probe (position 18 31))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "massLimit")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit")))))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 17 25) (end 17 34)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 25 38) (end 25 53)) (probe (position 25 38))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 29 37) (end 29 52)) (probe (position 29 37))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 7 21) (end 7 30)) (probe (position 7 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 11 21) (end 11 30)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 4 20) (end 4 29)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 21 38) (end 21 53)) (probe (position 21 38))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
)
~~~
