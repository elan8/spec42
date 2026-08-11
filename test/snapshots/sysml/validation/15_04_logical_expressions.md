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
  (document "15_04_logical_expressions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 2) (end 12 39))
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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "36e4d397f09e68d16279790019cd836d905b6b53ddd11ca4a8638e56c4f2f4f2") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (kind "package") (name "15_04-Logical Expressions") (declared-name "15_04-Logical Expressions") (range (start (line 0) (character 0)) (end (line 0) (character 724))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 32))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 28))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind "part def") (name "4CylEngine") (declared-name "4CylEngine") (range (start (line 4) (character 1)) (end (line 4) (character 33))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 4) (character 26)) (end (line 4) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind "part def") (name "6CylEngine") (declared-name "6CylEngine") (range (start (line 5) (character 1)) (end (line 5) (character 33))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine") (range (start (line 5) (character 26)) (end (line 5) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind "part def") (name "AutomaticTransmission") (declared-name "AutomaticTransmission") (range (start (line 9) (character 1)) (end (line 9) (character 48))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission") (range (start (line 9) (character 35)) (end (line 9) (character 47)))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 3) (character 1)) (end (line 3) (character 17))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind "part def") (name "ManualTransmission") (declared-name "ManualTransmission") (range (start (line 8) (character 1)) (end (line 8) (character 45))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission") (range (start (line 8) (character 32)) (end (line 8) (character 44)))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (range (start (line 7) (character 1)) (end (line 7) (character 23))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 11) (character 1)) (end (line 11) (character 440))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 14) (character 2)) (end (line 14) (character 25))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 14) (character 15)) (end (line 14) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind "attribute") (name "isHighPerformance") (declared-name "isHighPerformance") (range (start (line 12) (character 2)) (end (line 12) (character 39))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean") (range none)) (typing (reference "Boolean") (range (start (line 12) (character 31)) (end (line 12) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 15) (character 2)) (end (line 15) (character 37))) (parent (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission") (range (start (line 15) (character 21)) (end (line 15) (character 33)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (range (start (line 1) (character 16)) (end (line 1) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 4) (character 26)) (end (line 4) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (range (start (line 5) (character 26)) (end (line 5) (character 32))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (range (start (line 9) (character 35)) (end (line 9) (character 47))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (range (start (line 8) (character 32)) (end (line 8) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 14) (character 15)) (end (line 14) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (range (start (line 12) (character 31)) (end (line 12) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (range (start (line 15) (character 21)) (end (line 15) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission")))))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
