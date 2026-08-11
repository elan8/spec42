# META
~~~ini
description=SysML Training 32 (Requirements): Requirement Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Requirement Usages' {
	private import SI::*;
	private import 'Requirement Definitions'::*;
	
	requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 2000[kg];
		
		assume constraint {
			doc /* Full tank is full. */
			vehicle.fuelMass == vehicle.fuelFullMass
		}
	}
	
	requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
		subject vehicle : Vehicle;
		attribute :>> massReqd = 1500[kg];
		
		assume constraint {
			doc /* Full tank is empty. */
			vehicle.fuelMass == 0[kg]
		}
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "32_requirement_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 41))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 4 1) (end 4 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 1) (end 4 252))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 2) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 14 1) (end 14 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 1) (end 14 239))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 28))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Requirement Usages' {
    private import SI::*;
    private import 'Requirement Definitions'::*;

    requirement <'1.1'> fullVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        attribute :>> massReqd = 2000[kg];

        assume constraint {
            doc /* Full tank is full. */
            vehicle.fuelMass == vehicle.fuelFullMass
        }
    }

    requirement <'1.2'> emptyVehicleMassLimit : VehicleMassLimitationRequirement {
        subject vehicle : Vehicle;
        attribute :>> massReqd = 1500[kg];

        assume constraint {
            doc /* Full tank is empty. */
            vehicle.fuelMass == 0[kg]
        }
    }

}

~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a0ccd04064539f306ca7a9267dfc4a61479ed112d8718272a2771ac4f82b315c") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Requirement Usages"))) (kind "package") (name "Requirement Usages") (declared-name "Requirement Usages") (range (start (line 0) (character 0)) (end (line 0) (character 600))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 22))) (parent (node (document "d0") (qualified-name "Requirement Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 18))))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 45))) (parent (node (document "d0") (qualified-name "Requirement Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Requirement Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind "requirement") (name "emptyVehicleMassLimit") (declared-name "emptyVehicleMassLimit") (range (start (line 14) (character 1)) (end (line 14) (character 239))) (parent (node (document "d0") (qualified-name "Requirement Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleMassLimitationRequirement") (range none)) (subject (reference "Requirement Usages::emptyVehicleMassLimit::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 18) (character 2)) (end (line 18) (character 87))) (parent (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (range (start (line 18) (character 2)) (end (line 18) (character 87))) (parent (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 16) (character 2)) (end (line 16) (character 36))) (parent (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (authored (relationships (redefinition (reference "massReqd") (range (start (line 16) (character 16)) (end (line 16) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 15) (character 2)) (end (line 15) (character 28))) (parent (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind "requirement") (name "fullVehicleMassLimit") (declared-name "fullVehicleMassLimit") (range (start (line 4) (character 1)) (end (line 4) (character 252))) (parent (node (document "d0") (qualified-name "Requirement Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleMassLimitationRequirement") (range none)) (subject (reference "Requirement Usages::fullVehicleMassLimit::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 8) (character 2)) (end (line 8) (character 101))) (parent (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0::_documentation"))) (kind "documentation") (name "") (range (start (line 8) (character 2)) (end (line 8) (character 101))) (parent (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0"))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 6) (character 2)) (end (line 6) (character 36))) (parent (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (authored (relationships (redefinition (reference "massReqd") (range (start (line 6) (character 16)) (end (line 6) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 5) (character 2)) (end (line 5) (character 28))) (parent (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 1) (character 16)) (end (line 1) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Requirement Definitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleMassLimitationRequirement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Usages::emptyVehicleMassLimit::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (range (start (line 16) (character 16)) (end (line 16) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleMassLimitationRequirement") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Requirement Usages::fullVehicleMassLimit::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (range (start (line 6) (character 16)) (end (line 6) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (target (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd"))) (target (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd"))) (kind redefinition) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (target (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd"))) (target (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Requirement Usages::emptyVehicleMassLimit::massReqd")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Requirement Usages::fullVehicleMassLimit::massReqd")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
