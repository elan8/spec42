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
  (document "31_analytical_constraints.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 2) (end 22 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 2) (end 23 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 2) (end 24 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 25 2) (end 25 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 26 2) (end 26 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 2) (end 27 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 2) (end 28 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 29 2) (end 29 32))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "2e1952a783ac476627600549c28ed42542fff5d7924080e0dd29fd5e9a9c43ce") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Analytical Constraints"))) (kind "package") (name "Analytical Constraints") (declared-name "Analytical Constraints") (range (start (line 0) (character 0)) (end (line 0) (character 969))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "Analytical Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 45))) (parent (node (document "d0") (qualified-name "Analytical Constraints"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 41))))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (kind "action def") (name "StraightLineDynamics") (declared-name "StraightLineDynamics") (range (start (line 21) (character 1)) (end (line 21) (character 465))) (parent (node (document "d0") (qualified-name "Analytical Constraints"))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::a_out"))) (kind "in out parameter") (name "a_out") (declared-name "a_out") (range (start (line 29) (character 2)) (end (line 29) (character 32))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "AccelerationValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (range (start (line 24) (character 2)) (end (line 24) (character 25))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "TimeValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (range (start (line 23) (character 2)) (end (line 23) (character 22))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 22) (character 2)) (end (line 22) (character 24))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::v_in"))) (kind "in out parameter") (name "v_in") (declared-name "v_in") (range (start (line 26) (character 2)) (end (line 26) (character 23))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::v_out"))) (kind "in out parameter") (name "v_out") (declared-name "v_out") (range (start (line 28) (character 2)) (end (line 28) (character 25))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "SpeedValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::x_in"))) (kind "in out parameter") (name "x_in") (declared-name "x_in") (range (start (line 25) (character 2)) (end (line 25) (character 24))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::x_out"))) (kind "in out parameter") (name "x_out") (declared-name "x_out") (range (start (line 27) (character 2)) (end (line 27) (character 26))) (parent (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics"))) (authored (relationships (typing (reference "LengthValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations"))) (kind "constraint def") (name "StraightLineDynamicsEquations") (declared-name "StraightLineDynamicsEquations") (range (start (line 4) (character 1)) (end (line 4) (character 392))) (parent (node (document "d0") (qualified-name "Analytical Constraints"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 41))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::a_out"))) (kind featureTyping) (ordinal 0)) (authored-target "AccelerationValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::power"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::v_in"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::v_out"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::x_in"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamics::x_out"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
  )
  (relationships
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Analytical Constraints::StraightLineDynamicsEquations")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Analytical Constraints::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 41)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Analytical Constraints::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions::*")
        (range (start 2 16) (end 2 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
