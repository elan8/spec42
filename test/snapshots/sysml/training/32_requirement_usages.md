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
  (document "memory://snapshot/32_requirement_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 4 44) (end 4 76))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 5 2) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 6 16) (end 6 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 8 2) (end 11 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 45) (end 14 77))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 16 16) (end 16 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 18 2) (end 21 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:7330865c3611c2ae66217b7db3030ee73531944bf191d84e5f789335317febb9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Requirement Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassLimitationRequirement"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassLimitationRequirement"))))
    (declaration (id (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unsupported)))
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
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 1 16) (end 1 21)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 2 16) (end 2 44)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Requirement Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 14 45) (end 14 77)) (probe (position 14 45))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::emptyVehicleMassLimit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 16 16) (end 16 24)) (probe (position 16 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 4 44) (end 4 76)) (probe (position 4 44))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (qualified-name "Requirement Usages::fullVehicleMassLimit"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassLimitationRequirement")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/32_requirement_usages.md") (range (start 6 16) (end 6 24)) (probe (position 6 16))
    (reference (id (source (node (document "memory://snapshot/32_requirement_usages.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unsupported)))
  )
)
~~~
