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
  (document "vehicle_requirement_derivation.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 37))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 25) (end 17 34))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "efac69a83f704b583528df1af9f40fbe78ef991f17cdf0974598b98f396a64fc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (kind "package") (name "VehicleRequirementDerivation") (declared-name "VehicleRequirementDerivation"))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementDerivation::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (kind "requirement def") (name "MassRequirement") (declared-name "MassRequirement") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind "attribute") (name "massLimit") (declared-name "massLimit") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (authored (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (kind "derivation connection") (name "_derivationConnection") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (kind "interface end") (name "#derive") (declared-name "#derive") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "chassisMassRequirement")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (kind "interface end") (name "#derive") (declared-name "#derive") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "engineMassRequirement")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (kind "interface end") (name "#original") (declared-name "#original") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "vehicleMassRequirement")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind "requirement") (name "chassisMassRequirement") (declared-name "chassisMassRequirement") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind "requirement") (name "engineMassRequirement") (declared-name "engineMassRequirement") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis"))) (kind "part") (name "chassis") (declared-name "chassis") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementDerivation::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "chassisMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "engineMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "vehicleMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")) (expression (status "ambiguous") (error "expression has an ambiguous reference")) (analysis (status "ambiguous")))
    (node (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::_requireConstraint_0")) (expression (status "ambiguous") (error "expression has an ambiguous reference")))
    (node (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement")) (expression (status "ambiguous") (error "expression has an ambiguous reference")) (analysis (status "ambiguous")))
    (node (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement")) (expression (status "ambiguous") (error "expression has an ambiguous reference")) (analysis (status "ambiguous")))
    (node (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement")) (expression (status "ambiguous") (error "expression has an ambiguous reference")) (analysis (status "ambiguous")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 20) (end 4 29)) (probe (position 4 20))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 4 20) (end 4 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 7 21) (end 7 30)) (probe (position 7 21))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 7 21) (end 7 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 21) (end 11 30)) (probe (position 11 21))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 11 21) (end 11 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 17 25) (end 17 34)) (probe (position 17 25))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 17 25) (end 17 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 37)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "RequirementDerivation::*")
        (range (start 1 16) (end 1 37))
        (outcome (status unresolved))
      )
    )
    (query (range (start 36 18) (end 36 39)) (probe (position 36 18))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "engineMassRequirement")
        (range (start 36 18) (end 36 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement") (range (start 29 1) (end 29 99)))
        )
      )
    )
    (query (range (start 34 20) (end 34 42)) (probe (position 34 20))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "vehicleMassRequirement")
        (range (start 34 20) (end 34 42))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement") (range (start 21 1) (end 21 93)))
        )
      )
    )
    (query (range (start 35 18) (end 35 40)) (probe (position 35 18))
      (reference
        (source (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))
        (kind referenceSubsetting) (ordinal 0) (authored-target "chassisMassRequirement")
        (range (start 35 18) (end 35 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement") (range (start 25 1) (end 25 101)))
        )
      )
    )
  )
)
~~~
