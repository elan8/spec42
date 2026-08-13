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
  (document "memory://snapshot/36_variation_usages.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 16 2) (end 21 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:f72db2988c66ac92287bce6752db25cdb9612daf9e3c836aeb1851610af1e0ec") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Variation Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineChoices"))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission") (variation true)) (variant (reference "manualTransmission")) (variant (reference "automaticTransmission"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Variation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineChoices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind variant) (ordinal 0))
      (authored-target "manualTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind variant) (ordinal 1))
      (authored-target "automaticTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind variant) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind variant) (ordinal 0)))
    (relationship (kind variant) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind variant) (ordinal 1)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Variation Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 8 31) (end 8 38)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")))))
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 9 16) (end 9 29)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0) (authored-target "EngineChoices")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 11 32) (end 11 44)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")))))
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 12 11) (end 12 29)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind variant) (ordinal 0) (authored-target "manualTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission")))))
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 13 11) (end 13 32)) (probe (position 13 11))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind variant) (ordinal 1) (authored-target "automaticTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission")))))
  )
)
~~~
