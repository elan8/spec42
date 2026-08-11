# META
~~~ini
description=SysML Training 30 (Calculations): Calculation Usages-2
type=file
~~~
# SOURCE
~~~sysml
package 'Calculation Usages-2' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	attribute def DynamicState {
		attribute v: SpeedValue;
		attribute x: LengthValue;
	}
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		calc updateState { 
			in delta_t : TimeValue; 
			in currState : DynamicState;
			attribute totalPower : PowerValue = Power(wheelPower, C_d, C_f, mass, currState.v);
			
			return attribute newState : DynamicState {
				:>> v = Velocity(delta_t, currState.v, Acceleration(totalPower, mass, currState.v));
				:>> x = Position(delta_t, currState.x, currState.v);
			}
		}
	} 
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "30_calculation_usages_2.md"
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
        (range (start 2 16) (end 2 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 41))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 2) (end 6 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 2) (end 7 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 2) (end 13 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 13 25) (end 13 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 19) (end 14 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 3) (end 17 26))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Calculation Usages-2' {
	private import ScalarValues::Real;
	private import ISQ::*;
	private import 'Calculation Definitions'::*;
	
	attribute def DynamicState {
		attribute v: SpeedValue;
		attribute x: LengthValue;
	}
	
	part def VehicleDynamics {
		attribute C_d : Real;
		attribute C_f : Real;
		attribute wheelPower : PowerValue;
		attribute mass : MassValue;
		
		calc updateState { 
			in delta_t : TimeValue; 
			in currState : DynamicState;
			attribute totalPower : PowerValue = Power(wheelPower, C_d, C_f, mass, currState.v);
			
			return attribute newState : DynamicState {
				:>> v = Velocity(delta_t, currState.v, Acceleration(totalPower, mass, currState.v));
				:>> x = Position(delta_t, currState.x, currState.v);
			}
		}
	} 
	
}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5f40e41cb65a555e3b40eb51b11710aaeec07ff57fd3ba4caadafd191a51fdb8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2"))) (kind "package") (name "Calculation Usages-2") (declared-name "Calculation Usages-2"))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Calculation Usages-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Calculation Usages-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "Calculation Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (kind "attribute def") (name "DynamicState") (declared-name "DynamicState") (parent (node (document "d0") (qualified-name "Calculation Usages-2"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind "attribute") (name "v") (declared-name "v") (parent (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (authored (membership (kind Feature)) (relationships (typing (reference "SpeedValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind "attribute") (name "x") (declared-name "x") (parent (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Calculation Usages-2"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (kind "part def") (name "VehicleDynamics") (declared-name "VehicleDynamics") (parent (node (document "d0") (qualified-name "Calculation Usages-2"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind "attribute") (name "C_d") (declared-name "C_d") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind "attribute") (name "C_f") (declared-name "C_f") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "Real")) (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind "calc") (name "updateState") (declared-name "updateState") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind "in out parameter") (name "currState") (declared-name "currState") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (authored (relationships (typing (reference "DynamicState")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind "in out parameter") (name "delta_t") (declared-name "delta_t") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (authored (relationships (typing (reference "TimeValue")))))
    (element (id (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind "attribute") (name "wheelPower") (declared-name "wheelPower") (parent (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerValue")) (typing (reference "PowerValue")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Calculation Definitions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind featureTyping) (ordinal 0)) (authored-target "SpeedValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0)) (authored-target "DynamicState") (outcome (status resolved) (target (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState")))))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind featureTyping) (ordinal 0)) (authored-target "TimeValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 1)) (authored-target "PowerValue") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 1)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (target (node (document "d0") (qualified-name "Calculation Usages-2::DynamicState"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 19)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 2 16) (end 2 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 18) (end 11 22)) (probe (position 11 18))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 11 18) (end 11 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Calculation Usages-2::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 12 18) (end 12 22)) (probe (position 12 18))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))
        (kind featureTyping) (ordinal 1) (authored-target "Real")
        (range (start 12 18) (end 12 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Calculation Usages-2::Real") (range (start 1 1) (end 1 35)))
        )
      )
    )
    (query (range (start 14 19) (end 14 28)) (probe (position 14 19))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 14 19) (end 14 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 13 25) (end 13 35)) (probe (position 13 25))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))
        (kind featureTyping) (ordinal 1) (authored-target "PowerValue")
        (range (start 13 25) (end 13 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 41)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "Calculation Usages-2::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions::*")
        (range (start 3 16) (end 3 41))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
