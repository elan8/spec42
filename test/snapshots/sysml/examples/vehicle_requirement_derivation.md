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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 4 20) (end 4 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 7 21) (end 7 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
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
        (code "unsupported_reference")
        (source "semantic")
        (range (start 17 25) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 42))
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
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 33 1) (end 37 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:69413829c57e7c8016e64aecc70f3ed72bc64ca31c58aa323e9c50563e7f0493") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "RequirementDerivation") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
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
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
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
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 17 25) (end 17 34)) (probe (position 17 25))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
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
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 11 21) (end 11 30)) (probe (position 11 21))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 4 20) (end 4 29)) (probe (position 4 20))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/vehicle_requirement_derivation.md") (range (start 21 38) (end 21 53)) (probe (position 21 38))
    (reference (id (source (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/vehicle_requirement_derivation.md") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
)
~~~
