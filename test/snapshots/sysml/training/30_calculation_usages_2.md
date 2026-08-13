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
  (document "memory://snapshot/30_calculation_usages_2.md"
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
        (range (start 6 15) (end 6 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 15) (end 7 26))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 18) (end 11 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 12 18) (end 12 22))
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
        (range (start 14 19) (end 14 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 16) (end 17 25))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 3) (end 19 12))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 13) (end 19 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 19 24) (end 21 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 10) (end 21 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 21 20) (end 21 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 21 29) (end 25 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:8845fda36be87b67f5235ca54c41e890d079eda091444f67733f3a3957cfcb86") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Calculation Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "attribute")) (expressionOperand (reference "totalPower")) (expressionOperand (reference "attribute")) (expressionOperand (reference "newState"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DynamicState") (direction in))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "SpeedValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 0))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 1))
      (authored-target "totalPower")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 2))
      (authored-target "attribute")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 3))
      (authored-target "newState")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0))
      (authored-target "DynamicState")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 3 16) (end 3 44)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 6 15) (end 6 25)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind featureTyping) (ordinal 0) (authored-target "SpeedValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 7 15) (end 7 26)) (probe (position 7 15))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 11 18) (end 11 22)) (probe (position 11 18))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 12 18) (end 12 22)) (probe (position 12 18))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 14 19) (end 14 28)) (probe (position 14 19))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 3) (end 19 12)) (probe (position 19 3))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 0) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 13) (end 19 23)) (probe (position 19 13))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 1) (authored-target "totalPower")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 21 10) (end 21 19)) (probe (position 21 10))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 2) (authored-target "attribute")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 21 20) (end 21 28)) (probe (position 21 20))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind expressionOperand) (ordinal 3) (authored-target "newState")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 18 18) (end 18 30)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0) (authored-target "DynamicState")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 17 16) (end 17 25)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 13 25) (end 13 35)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
)
~~~
