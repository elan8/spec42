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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 26) (end 19 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 39) (end 19 44))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation false) (source-digest "blake3:8845fda36be87b67f5235ca54c41e890d079eda091444f67733f3a3957cfcb86") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 0)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 1)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 2)))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Calculation Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState"))) (kind attribute-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "SpeedValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::x"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DynamicState") (direction in))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TimeValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::newState"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DynamicState"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue")) (expressionOperand (reference "wheelPower")) (expressionOperand (reference "C_d")) (expressionOperand (reference "C_f")) (expressionOperand (reference "mass")) (memberAccessOperand (reference "currState::v")) (invocationCallee (reference "Power"))))
    (declaration (id (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerValue"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0))
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
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0))
      (authored-target "DynamicState")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind featureTyping) (ordinal 0))
      (authored-target "TimeValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::newState"))) (kind featureTyping) (ordinal 0))
      (authored-target "DynamicState")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 0))
      (authored-target "wheelPower")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 1))
      (authored-target "C_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 2))
      (authored-target "C_f")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 3))
      (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "currState::v")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v")))))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind invocationCallee) (ordinal 0))
      (authored-target "Power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::newState"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::newState"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 2)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 3)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind memberAccessOperand) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 2 16) (end 2 22)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 1)))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 3 16) (end 3 44)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 2)))))) (kind namespaceImport) (ordinal 0) (authored-target "Calculation Definitions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (path (named (kind package) (name "Calculation Usages-2")) (anonymous (kind import) (ordinal 0)))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
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
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 18 18) (end 18 30)) (probe (position 18 18))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::currState"))) (kind featureTyping) (ordinal 0) (authored-target "DynamicState")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 17 16) (end 17 25)) (probe (position 17 16))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::delta_t"))) (kind featureTyping) (ordinal 0) (authored-target "TimeValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 21 31) (end 21 43)) (probe (position 21 31))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::newState"))) (kind featureTyping) (ordinal 0) (authored-target "DynamicState")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 26) (end 19 36)) (probe (position 19 26))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 45) (end 19 55)) (probe (position 19 45))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 0) (authored-target "wheelPower")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 57) (end 19 60)) (probe (position 19 57))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 1) (authored-target "C_d")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_d")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 62) (end 19 65)) (probe (position 19 62))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 2) (authored-target "C_f")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::C_f")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 67) (end 19 71)) (probe (position 19 67))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind expressionOperand) (ordinal 3) (authored-target "mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::mass")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 73) (end 19 84)) (probe (position 19 73))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind memberAccessOperand) (ordinal 0) (authored-target "currState::v")
      (outcome (status resolved) (target (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::DynamicState::v")))))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 19 39) (end 19 44)) (probe (position 19 39))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::updateState::totalPower"))) (kind invocationCallee) (ordinal 0) (authored-target "Power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/30_calculation_usages_2.md") (range (start 13 25) (end 13 35)) (probe (position 13 25))
    (reference (id (source (node (document "memory://snapshot/30_calculation_usages_2.md") (qualified-name "Calculation Usages-2::VehicleDynamics::wheelPower"))) (kind featureTyping) (ordinal 0) (authored-target "PowerValue")
      (outcome (status unresolved)))
  )
)
~~~
