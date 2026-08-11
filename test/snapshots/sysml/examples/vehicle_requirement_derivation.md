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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "efac69a83f704b583528df1af9f40fbe78ef991f17cdf0974598b98f396a64fc") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (kind "package") (name "VehicleRequirementDerivation") (declared-name "VehicleRequirementDerivation") (range (start (line 0) (character 0)) (end (line 0) (character 855))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 41))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Import) (visibility "private") (import (reference "RequirementDerivation::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 37))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (kind "requirement def") (name "MassRequirement") (declared-name "MassRequirement") (range (start (line 15) (character 1)) (end (line 15) (character 145))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 18) (character 2)) (end (line 18) (character 42))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind "attribute") (name "massLimit") (declared-name "massLimit") (range (start (line 17) (character 2)) (end (line 17) (character 35))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement"))) (authored (relationships (subsetting (reference "ISQ::mass") (range (start (line 17) (character 25)) (end (line 17) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (kind "derivation connection") (name "_derivationConnection") (range (start (line 33) (character 1)) (end (line 33) (character 155))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (kind "interface end") (name "#derive") (declared-name "#derive") (range (start (line 35) (character 2)) (end (line 35) (character 41))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "chassisMassRequirement") (range (start (line 35) (character 18)) (end (line 35) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (kind "interface end") (name "#derive") (declared-name "#derive") (range (start (line 36) (character 2)) (end (line 36) (character 40))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "engineMassRequirement") (range (start (line 36) (character 18)) (end (line 36) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (kind "interface end") (name "#original") (declared-name "#original") (range (start (line 34) (character 2)) (end (line 34) (character 43))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection"))) (authored (relationships (reference-subsetting (reference "vehicleMassRequirement") (range (start (line 34) (character 20)) (end (line 34) (character 42)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind "requirement") (name "chassisMassRequirement") (declared-name "chassisMassRequirement") (range (start (line 25) (character 1)) (end (line 25) (character 101))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind "requirement") (name "engineMassRequirement") (declared-name "engineMassRequirement") (range (start (line 29) (character 1)) (end (line 29) (character 99))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 3) (character 1)) (end (line 3) (character 160))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis"))) (kind "part") (name "chassis") (declared-name "chassis") (range (start (line 6) (character 2)) (end (line 6) (character 52))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 7) (character 3)) (end (line 7) (character 31))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 7) (character 21)) (end (line 7) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 10) (character 2)) (end (line 10) (character 51))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 11) (character 3)) (end (line 11) (character 31))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 11) (character 21)) (end (line 11) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 4) (character 2)) (end (line 4) (character 30))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 4) (character 20)) (end (line 4) (character 29)))))))
    (element (id (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (range (start (line 21) (character 1)) (end (line 21) (character 93))) (parent (node (document "d0") (qualified-name "VehicleRequirementDerivation"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "RequirementDerivation::*") (range (start (line 1) (character 16)) (end (line 1) (character 37))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement::massLimit"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 17) (character 25)) (end (line 17) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "chassisMassRequirement") (range (start (line 35) (character 18)) (end (line 35) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#derive#interface_end"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "engineMassRequirement") (range (start (line 36) (character 18)) (end (line 36) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::_derivationConnection::#original"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "vehicleMassRequirement") (range (start (line 34) (character 20)) (end (line 34) (character 42))) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::chassisMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::engineMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::chassis::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 7) (character 21)) (end (line 7) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::engine::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 11) (character 21)) (end (line 11) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 4) (character 20)) (end (line 4) (character 29))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "VehicleRequirementDerivation::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "VehicleRequirementDerivation::MassRequirement")))))
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
