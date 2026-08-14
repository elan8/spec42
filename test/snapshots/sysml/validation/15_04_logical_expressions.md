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
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:031290a29b10c4622d040a83e6bc04b0e4b6eed91bbbed2db901bf7e91f277af") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ScalarValues") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Transmission")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "isHighPerformance")) (expressionOperand (reference "engine")) (expressionOperand (reference "engine")) (typeCheckTarget (reference "6CylEngine")) (typeCheckTarget (reference "4CylEngine")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind assert-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engine")) (expressionOperand (reference "transmission")) (expressionOperand (reference "engine")) (expressionOperand (reference "transmission")) (typeCheckTarget (reference "4CylEngine")) (typeCheckTarget (reference "ManualTransmission")) (typeCheckTarget (reference "6CylEngine")) (typeCheckTarget (reference "AutomaticTransmission")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Boolean")))))
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)) (facts (multiplicity (lower 1) (upper 1))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Transmission")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "isHighPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 1))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 2))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 3))
      (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "6CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 0))
      (authored-target "4CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind typeCheckTarget) (ordinal 1))
      (authored-target "4CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 1))
      (authored-target "ManualTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 2))
      (authored-target "6CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine")))))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 3))
      (authored-target "AutomaticTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission")))))
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
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 0)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind typeCheckTarget) (ordinal 1)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 1)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 2)))
    (relationship (kind typeCheckTarget) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 3)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine")))
      (supertype (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine")))
      (supertype (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission")))
      (supertype (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission")))
      (supertype (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")) (scopes any subclassification))
    )
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))
      (supertype (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission")))
      (supertype (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 1 16) (end 1 31)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 4 26) (end 4 32)) (probe (position 4 26))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 5 26) (end 5 32)) (probe (position 5 26))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 9 35) (end 9 47)) (probe (position 9 35))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 8 32) (end 8 44)) (probe (position 8 32))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 18 6) (end 18 23)) (probe (position 18 6))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "isHighPerformance")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 23 4) (end 23 10)) (probe (position 23 4))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 18 25) (end 18 31)) (probe (position 18 25))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 24 4) (end 24 16)) (probe (position 24 4))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 1) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 19 8) (end 19 14)) (probe (position 19 8))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 2) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 25 4) (end 25 10)) (probe (position 25 4))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 2) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 26 4) (end 26 16)) (probe (position 26 4))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind expressionOperand) (ordinal 3) (authored-target "transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 18 39) (end 18 51)) (probe (position 18 39))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind typeCheckTarget) (ordinal 0) (authored-target "6CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 23 18) (end 23 30)) (probe (position 23 18))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 0) (authored-target "4CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 19 22) (end 19 34)) (probe (position 19 22))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 0))))) (kind typeCheckTarget) (ordinal 1) (authored-target "4CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::4CylEngine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 24 24) (end 24 42)) (probe (position 24 24))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 1) (authored-target "ManualTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::ManualTransmission")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 25 18) (end 25 30)) (probe (position 25 18))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 2) (authored-target "6CylEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::6CylEngine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 26 24) (end 26 45)) (probe (position 26 24))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (path (named (kind package) (name "15_04-Logical Expressions")) (named (kind part-def) (name "Vehicle")) (anonymous (kind assert-constraint) (ordinal 1))))) (kind typeCheckTarget) (ordinal 3) (authored-target "AutomaticTransmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::AutomaticTransmission")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 14 15) (end 14 21)) (probe (position 14 15))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Engine")))))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 12 31) (end 12 38)) (probe (position 12 31))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 0) (authored-target "Boolean")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/15_04_logical_expressions.md") (range (start 15 21) (end 15 33)) (probe (position 15 21))
    (reference (id (source (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0) (authored-target "Transmission")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_04_logical_expressions.md") (qualified-name "15_04-Logical Expressions::Transmission")))))
    )
  )
)
~~~
