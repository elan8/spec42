# META
~~~ini
description=SysML Training 36 (Variability): Variation Usages
type=file
~~~
# SOURCE
~~~sysml
package 'Variation Usages' {
	private import 'Variation Definitions'::*;
	
	part def Vehicle;
	part def Transmission;
	part manualTransmission;
	part automaticTransmission;
	
	abstract part vehicleFamily : Vehicle {
		part engine : EngineChoices[1];
		
		variation part transmission : Transmission[1] {
			variant manualTransmission;
			variant automaticTransmission;
		}
		
		assert constraint {
			(engine == engine::'4cylEngine' and
			 transmission == transmission::manualTransmission) xor
			(engine == engine::'6cylEngine' and 
			 transmission == transmission::automaticTransmission)
		}	
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "36_variation_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 39))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 5 1) (end 5 25))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "sysml")
        (range (start 6 1) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 29))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "dd2905b5ded6a6eb6e0d2a6f2edcb5b46d61a365b181a73103cffd3b12ea2209") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Variation Usages"))) (kind "package") (name "Variation Usages") (declared-name "Variation Usages"))
    (element (id (node (document "d0") (qualified-name "Variation Usages::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Variation Usages"))) (authored (membership (kind Import) (visibility "private") (import (reference "Variation Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::automaticTransmission"))) (kind "part") (name "automaticTransmission") (declared-name "automaticTransmission") (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::manualTransmission"))) (kind "part") (name "manualTransmission") (declared-name "manualTransmission") (parent (node (document "d0") (qualified-name "Variation Usages"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (kind "part") (name "vehicleFamily") (declared-name "vehicleFamily") (parent (node (document "d0") (qualified-name "Variation Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "EngineChoices")))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission::automaticTransmission"))) (kind "variant") (name "automaticTransmission") (declared-name "automaticTransmission") (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))))
    (element (id (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission::manualTransmission"))) (kind "variant") (name "manualTransmission") (declared-name "manualTransmission") (parent (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Variation Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Usages::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "EngineChoices") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "Variation Usages::Transmission")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (target (node (document "d0") (qualified-name "Variation Usages::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "d0") (qualified-name "Variation Usages::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 8 31) (end 8 38)) (probe (position 8 31))
      (reference
        (source (document "d0") (qualified-name "Variation Usages::vehicleFamily"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 8 31) (end 8 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Usages::Vehicle") (range (start 3 1) (end 3 18)))
        )
      )
    )
    (query (range (start 11 32) (end 11 44)) (probe (position 11 32))
      (reference
        (source (document "d0") (qualified-name "Variation Usages::vehicleFamily::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 11 32) (end 11 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Variation Usages::Transmission") (range (start 4 1) (end 4 23)))
        )
      )
    )
    (query (range (start 9 16) (end 9 29)) (probe (position 9 16))
      (reference
        (source (document "d0") (qualified-name "Variation Usages::vehicleFamily::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "EngineChoices")
        (range (start 9 16) (end 9 29))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 39)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Variation Usages::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Variation Definitions::*")
        (range (start 1 16) (end 1 39))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
