# META
~~~ini
description=SysML Training 31 (Constraints): Analytical Constraints
type=file
~~~
# SOURCE
~~~sysml
package 'Analytical Constraints' {
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	constraint def StraightLineDynamicsEquations {
		in p : PowerValue;
		in m : MassValue;
		in dt : TimeValue;
		in x_i : LengthValue;
		in v_i : SpeedValue;
		in x_f : LengthValue;
		in v_f : SpeedValue;
		in a : AccelerationValue;
	
		attribute v_avg : SpeedValue = (v_i + v_f)/2;
		
		a == Acceleration(p, m, v_avg) and
		v_f == Velocity(dt, v_i, a) and
		x_f == Position(dt, x_i, v_avg)
	}
	
	action def StraightLineDynamics {
		in power : PowerValue;
		in mass : MassValue;
		in delta_t : TimeValue;
		in x_in : LengthValue;
		in v_in : SpeedValue;
		out x_out : LengthValue;
		out v_out : SpeedValue;
		out a_out : AccelerationValue;
	
	    assert constraint dynamics : StraightLineDynamicsEquations {
			in p = power;
			in m = mass;
			in dt = delta_t;
			in x_i = x_in;
			in v_i = v_in;
			in x_f = x_out;
			in v_f = v_out;
			in a = a_out;
	    }
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/31_analytical_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
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
        (range (start 5 9) (end 5 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 9) (end 6 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 10) (end 7 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 11) (end 8 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 11) (end 9 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 11) (end 10 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 11) (end 11 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 9) (end 12 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 2) (end 14 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 12) (end 14 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 14 18) (end 16 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 16 2) (end 18 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 13) (end 22 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 12) (end 23 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 15) (end 24 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 12) (end 25 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 12) (end 26 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 14) (end 27 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 14) (end 28 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 14) (end 29 31))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_action_definition_member")
        (source "semantic")
        (range (start 31 5) (end 40 6))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:38f93a877fba3054bdff73b329c39c14b3803a9f07cca4d081f02ea3d71c33a3") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Calculation Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (kind action-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::a_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::delta_t"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::mass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::power"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::v_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::v_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::x_in"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::x_out"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction out))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (kind constraint-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (expressionOperand (reference "attribute")) (expressionOperand (reference "v_avg"))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::a"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AccelerationValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::dt"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::m"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::p"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::v_f"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::v_i"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::x_f"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::x_i"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue") (direction in))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (kind expressionOperand) (ordinal 0))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (kind expressionOperand) (ordinal 1))
      (authored-target "v_avg")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::a"))) (kind featureTyping) (ordinal 0))
      (authored-target "AccelerationValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::dt"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::m"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::v_f"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::v_i"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::x_f"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::x_i"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
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
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 2 16) (end 2 44)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 29 14) (end 29 31)) (probe (position 29 14))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 24 15) (end 24 24)) (probe (position 24 15))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 23 12) (end 23 21)) (probe (position 23 12))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 22 13) (end 22 23)) (probe (position 22 13))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 26 12) (end 26 22)) (probe (position 26 12))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 28 14) (end 28 24)) (probe (position 28 14))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 25 12) (end 25 23)) (probe (position 25 12))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 27 14) (end 27 25)) (probe (position 27 14))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 14 2) (end 14 11)) (probe (position 14 2))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (kind expressionOperand) (ordinal 0) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 14 12) (end 14 17)) (probe (position 14 12))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (kind expressionOperand) (ordinal 1) (authored-target "v_avg")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 12 9) (end 12 26)) (probe (position 12 9))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::a"))) (kind featureTyping) (ordinal 0) (authored-target "AccelerationValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 7 10) (end 7 19)) (probe (position 7 10))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::dt"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 6 9) (end 6 18)) (probe (position 6 9))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::m"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 5 9) (end 5 19)) (probe (position 5 9))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::p"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 11 11) (end 11 21)) (probe (position 11 11))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::v_f"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 9 11) (end 9 21)) (probe (position 9 11))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::v_i"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 10 11) (end 10 22)) (probe (position 10 11))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::x_f"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/31_analytical_constraints.md") (range (start 8 11) (end 8 22)) (probe (position 8 11))
    (reference (id (source (node (document "memory://snapshot/31_analytical_constraints.md") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations::x_i"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
)
~~~
