# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Definitions
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Definitions' {
	private import ScalarValues::Real;
	private import ISQ::*;
	
	calc def Power { in whlpwr : PowerValue; in Cd : Real; in Cf : Real; in tm : MassValue; in v : SpeedValue;
		attribute drag = Cd * v;
		attribute friction = Cf * tm * v;
		
		return : PowerValue = whlpwr - drag - friction;
	}
	
	calc def Acceleration { in tp: PowerValue; in tm : MassValue; in v : SpeedValue;
		return : AccelerationValue = tp / (tm * v);
	}
	
	calc def Velocity { in dt : TimeValue; in v0 : SpeedValue; in a : AccelerationValue;
		return : SpeedValue = v0 + a * dt;
 	}
 	
	calc def Position { in dt : TimeValue; in x0 : LengthValue; in v : SpeedValue;
		return : LengthValue = x0 + v * dt;
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/30_calculation_definitions.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 4 18) (end 4 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 4 42) (end 4 55))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 4 56) (end 4 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 4 70) (end 4 88))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 4 89) (end 4 107))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 5 2) (end 5 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 5 12) (end 5 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 5 17) (end 6 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 6 2) (end 6 11))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 6 12) (end 6 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 6 21) (end 8 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 8 2) (end 8 49))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 25) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 44) (end 11 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 63) (end 11 81))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 12 2) (end 12 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 21) (end 15 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 40) (end 15 59))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 60) (end 15 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 21) (end 19 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 40) (end 19 60))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 61) (end 19 79))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 20 2) (end 20 37))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:5c649927f6fe2bb32ac760b92a33d0e0ef146edb51749cbdb1012ba0cad7b917") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (qualified-name "Calculation Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (qualified-name "Calculation Definitions::Acceleration"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (qualified-name "Calculation Definitions::Position"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (qualified-name "Calculation Definitions::Power"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_definitions.md") (qualified-name "Calculation Definitions::Velocity"))) (kind calc-def) (membership (kind owning) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/30_calculation_definitions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_definitions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
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
  (query (document "memory://snapshot/30_calculation_definitions.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_definitions.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_definitions.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_definitions.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
)
~~~
