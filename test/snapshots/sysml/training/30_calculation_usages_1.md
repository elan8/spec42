# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Usages-1
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Usages-1' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		action straightLineDynamics {
			in delta_t : TimeValue;
			in v_in : SpeedValue;
			in x_in : LengthValue;
			out v_out : SpeedValue = vel.v;
			out x_out : LengthValue = pos.x;
		
			calc acc : Acceleration {
				in tp = Power(wheelPower, C_d, C_f, mass, v_in);
				in tm = mass;
				in v = v_in;
				return a;
			}
			
			calc vel : Velocity {
				in dt = delta_t;
				in v0 = v_in;
				in a = acc.a;
				return v;
			}
			
			calc pos : Position {
				in dt = delta_t;
				in x0 = x_in;
				in v0 = vel.v;
				return x;	
			}
		}
	} 
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/30_calculation_usages_1.md"
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
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 18) (end 6 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 18) (end 7 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 25) (end 8 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 19) (end 9 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 11 2) (end 38 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:3aed65b9cd3287667c670eab4c65edeb9e5730390c602a27457a65c4f10be039") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Calculation Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
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
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 3 16) (end 3 44)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 6 18) (end 6 22)) (probe (position 6 18))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 7 18) (end 7 22)) (probe (position 7 18))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 9 19) (end 9 28)) (probe (position 9 19))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_1.md") (range (start 8 25) (end 8 35)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_1.md") (qualified-name "Calculation Usages-1::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
)
~~~
