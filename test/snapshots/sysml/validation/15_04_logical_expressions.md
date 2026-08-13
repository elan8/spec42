# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_04-Logical Expressions
type=file
~~~
# SOURCE
~~~sysml
package '15_04-Logical Expressions' {
	private import ScalarValues::*;
	
	part def Engine;
	part def '4CylEngine' :> Engine;
	part def '6CylEngine' :> Engine;
	
	part def Transmission;
	part def ManualTransmission :> Transmission;
	part def AutomaticTransmission :> Transmission;
	
	part def Vehicle {
		attribute isHighPerformance: Boolean;
		
		part engine: Engine[1];
		part transmission: Transmission[1];
		
		assert constraint {
			if isHighPerformance? engine istype '6CylEngine'
			else engine istype '4CylEngine'
		}
		
		assert constraint {
			(engine istype '4CylEngine' and 
			 transmission istype ManualTransmission) xor
			(engine istype '6CylEngine' and
			 transmission istype AutomaticTransmission)
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_04_logical_expressions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 31) (end 12 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 17 2) (end 20 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 22 2) (end 27 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:031290a29b10c4622d040a83e6bc04b0e4b6eed91bbbed2db901bf7e91f277af") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission"))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean"))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ScalarValues")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 0))
      (authored-target "Boolean")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0))
      (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 4 26) (end 4 32)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 5 26) (end 5 32)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 9 35) (end 9 47)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 8 32) (end 8 44)) (probe (position 8 32))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 14 15) (end 14 21)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 12 31) (end 12 38)) (probe (position 12 31))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 15 21) (end 15 33)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
  )
)
~~~
