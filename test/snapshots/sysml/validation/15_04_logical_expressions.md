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
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (kind "package") (name "15_04-Logical Expressions") (declared-name "15_04-Logical Expressions"))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind "part def") (name "4CylEngine") (declared-name "4CylEngine") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind "part def") (name "6CylEngine") (declared-name "6CylEngine") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind "part def") (name "AutomaticTransmission") (declared-name "AutomaticTransmission") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind "part def") (name "ManualTransmission") (declared-name "ManualTransmission") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Transmission")))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission"))) (kind "part def") (name "Transmission") (declared-name "Transmission") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions"))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind "attribute") (name "isHighPerformance") (declared-name "isHighPerformance") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Boolean")) (typing (reference "Boolean")))))
    (element (id (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "Transmission")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ScalarValues::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))) (kind specialization) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))) (kind specialization) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 0)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))) (kind featureTyping) (ordinal 1)) (authored-target "Boolean") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))) (kind featureTyping) (ordinal 0)) (authored-target "Transmission") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_04-Logical Expressions::Transmission")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 4 26) (end 4 32)) (probe (position 4 26))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::4CylEngine"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 4 26) (end 4 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_04-Logical Expressions::Engine") (range (start 3 1) (end 3 17)))
        )
      )
    )
    (query (range (start 5 26) (end 5 32)) (probe (position 5 26))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::6CylEngine"))
        (kind specialization) (ordinal 0) (authored-target "Engine")
        (range (start 5 26) (end 5 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_04-Logical Expressions::Engine") (range (start 3 1) (end 3 17)))
        )
      )
    )
    (query (range (start 14 15) (end 14 21)) (probe (position 14 15))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 14 15) (end 14 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_04-Logical Expressions::Engine") (range (start 3 1) (end 3 17)))
        )
      )
    )
    (query (range (start 12 31) (end 12 38)) (probe (position 12 31))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::isHighPerformance"))
        (kind featureTyping) (ordinal 1) (authored-target "Boolean")
        (range (start 12 31) (end 12 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 28)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ScalarValues::*")
        (range (start 1 16) (end 1 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 32) (end 8 44)) (probe (position 8 32))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::ManualTransmission"))
        (kind specialization) (ordinal 0) (authored-target "Transmission")
        (range (start 8 32) (end 8 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_04-Logical Expressions::Transmission") (range (start 7 1) (end 7 23)))
        )
      )
    )
    (query (range (start 9 35) (end 9 47)) (probe (position 9 35))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::AutomaticTransmission"))
        (kind specialization) (ordinal 0) (authored-target "Transmission")
        (range (start 9 35) (end 9 47))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_04-Logical Expressions::Transmission") (range (start 7 1) (end 7 23)))
        )
      )
    )
    (query (range (start 15 21) (end 15 33)) (probe (position 15 21))
      (reference
        (source (document "d0") (qualified-name "15_04-Logical Expressions::Vehicle::transmission"))
        (kind featureTyping) (ordinal 0) (authored-target "Transmission")
        (range (start 15 21) (end 15 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_04-Logical Expressions::Transmission") (range (start 7 1) (end 7 23)))
        )
      )
    )
  )
)
~~~
