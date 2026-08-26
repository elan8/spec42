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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 42))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 5 1) (end 5 25))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 6 1) (end 6 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 16) (end 9 29))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 16 2) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 14) (end 17 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 18 20) (end 18 52))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 14) (end 19 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 20 20) (end 20 55))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f72db2988c66ac92287bce6752db25cdb9612daf9e3c836aeb1851610af1e0ec") (contract-version "constructor-expression-result-v8"))
  (declarations
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Variation Definitions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers abstract)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engine")) (expressionOperand (reference "engine::4cylEngine")) (expressionOperand (reference "transmission")) (expressionOperand (reference "transmission::manualTransmission")) (expressionOperand (reference "engine")) (expressionOperand (reference "engine::6cylEngine")) (expressionOperand (reference "transmission")) (expressionOperand (reference "transmission::automaticTransmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EngineChoices")))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind part) (membership (kind feature) (visibility default)) (facts (modifiers variation) (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission") (variation true)))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0))))) (kind ref) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (subsetting (reference "manualTransmission")))))
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1))))) (kind ref) (membership (kind owning) (visibility default) (role variant)) (authored (membership (kind owning) (visibility default) (role variant)) (relationships (subsetting (reference "automaticTransmission")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Variation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "engine::4cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 3))
      (authored-target "transmission::manualTransmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 4))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 5))
      (authored-target "engine::6cylEngine")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 6))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 7))
      (authored-target "transmission::automaticTransmission")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "EngineChoices")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "manualTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission")))))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1))))) (kind subsetting) (ordinal 0))
      (authored-target "automaticTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 4)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 6)))
    (relationship (kind typing) (variation true) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1))))) (kind subsetting) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (state unresolved-operand))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")))
      (subtype (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")))
      (subtype (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission")))
      (subtype (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission")))
      (subtype (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily")))
      (type (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine")))
      (featured-by (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily")))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission")))
      (featured-by (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily")))
      (type (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")) (provenance authored))
      (effective-type (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")) (source direct))
      (supertype (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0)))))
      (supertype (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1)))))
      (supertype (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission")) (scopes any feature))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 1 16) (end 1 42)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Variation Definitions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 8 31) (end 8 38)) (probe (position 8 31))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 17 4) (end 17 10)) (probe (position 17 4))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 17 14) (end 17 34)) (probe (position 17 14))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "engine::4cylEngine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 18 4) (end 18 16)) (probe (position 18 4))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 18 20) (end 18 52)) (probe (position 18 20))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 3) (authored-target "transmission::manualTransmission")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 19 4) (end 19 10)) (probe (position 19 4))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 4) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 19 14) (end 19 34)) (probe (position 19 14))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 5) (authored-target "engine::6cylEngine")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 20 4) (end 20 16)) (probe (position 20 4))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 6) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 20 20) (end 20 55)) (probe (position 20 20))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 7) (authored-target "transmission::automaticTransmission")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 9 16) (end 9 29)) (probe (position 9 16))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::engine"))) (kind featureTyping) (ordinal 0) (authored-target "EngineChoices")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 11 32) (end 11 44)) (probe (position 11 32))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::vehicleFamily::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::Transmission")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 12 11) (end 12 29)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "manualTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::manualTransmission")))))
    )
  )
  (query (document "memory://snapshot/36_variation_usages.md") (range (start 13 11) (end 13 32)) (probe (position 13 11))
    (reference (id (source (node (document "memory://snapshot/36_variation_usages.md") (path (named (kind package) (name "Variation Usages")) (named (kind part) (name "vehicleFamily")) (named (kind part) (name "transmission")) (anonymous (kind ref) (ordinal 1))))) (kind subsetting) (ordinal 0) (authored-target "automaticTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/36_variation_usages.md") (qualified-name "Variation Usages::automaticTransmission")))))
    )
  )
)
~~~
