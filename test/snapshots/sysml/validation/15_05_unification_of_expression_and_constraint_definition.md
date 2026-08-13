# META
~~~ini
description=SysML Validation (15-Properties-Values-Expressions): 15_05-Unification of Expression and Constraint Definition
type=file
~~~
# SOURCE
~~~sysml
package '15_05-Unification of Expression and Constraint Definition' {
	private import '15_03-Value Expression'::*;
	private import ControlFunctions::forAll;
	private import SI::*;
	
	constraint def DiscBrakeConstraint {
		in wheelAssy : WheelAssy[4];
		
		wheelAssy->forAll {in ref w: WheelAssy; 
			2 * w.discBrakeAssy.radius < w.wheel.outerDiameter
		}
	}
	
	constraint def DiscBrakeFitConstraint_Alt {
		in discBrakeAssy : DiscBrakeAssy[1];
		in wheel : Wheel[1];	
			
		2 * discBrakeAssy.radius < wheel.outerDiameter
	}
	
	part def Vehicle_2 {
		attribute mass : MassValue[1] = 1200 [kg];
		attribute length : LengthValue[1] = 4.82 [m];
		
		part wheelAssy : WheelAssy[4];
		
		constraint discBrakeConstraint : DiscBrakeConstraint {
			doc
			/*
			 * This constraint is computed, but not asserted. This means a tool can identify 
			 * when it is violated without the model being inconsistent.
			 */
			in wheelAssy = Vehicle_2::wheelAssy;
		}
	}
	
	part def WheelAssy {
		part wheel : Wheel[1];
		part discBrakeAssy : DiscBrakeAssy[1];
		
		assert constraint discBrakeFitConstraint_Alt: DiscBrakeFitConstraint_Alt {
			doc
			/*
			 * This constraint is asserted to be true, which means that the model
			 * is inconsistent if it the constraint is violated.
			 */
		
			in discBrakeAssy = WheelAssy::discBrakeAssy;
			in wheel = WheelAssy::wheel;
		}
	}
	
	part def DiscBrakeAssy {
		attribute radius : LengthValue[1] = 95 [mm];
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 8 2) (end 10 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 13) (end 15 18))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_constraint_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 48))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 19) (end 21 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 21) (end 22 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 15) (end 37 20))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 40 2) (end 49 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 21) (end 53 32))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:4e0e75715f7b7e9981466f3e551d5424bb7ddfc87d0548631bbd8c29dc97f32a") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "15_03-Value Expression") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ControlFunctions::forAll") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint::wheelAssy"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelAssy") (direction in))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt"))) (kind constraint-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::discBrakeAssy"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiscBrakeAssy") (direction in))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::wheel"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel") (direction in))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::discBrakeConstraint"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiscBrakeConstraint"))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::discBrakeConstraint::wheelAssy"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "LengthValue"))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue"))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WheelAssy"))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "DiscBrakeAssy"))))
    (declaration (id (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Wheel"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "15_03-Value Expression")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0))
      (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint::wheelAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::discBrakeAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiscBrakeAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy")))))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::discBrakeConstraint"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiscBrakeConstraint")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint")))))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind featureTyping) (ordinal 0))
      (authored-target "LengthValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "WheelAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind featureTyping) (ordinal 0))
      (authored-target "DiscBrakeAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy")))))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind featureTyping) (ordinal 0))
      (authored-target "Wheel")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint::wheelAssy"))) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint::wheelAssy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (direction in) (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::discBrakeAssy"))) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::discBrakeAssy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::discBrakeConstraint"))) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::discBrakeConstraint"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 1 16) (end 1 43)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "15_03-Value Expression")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 3 16) (end 3 21)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 2 16) (end 2 40)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (anonymous (kind import) (ordinal 1))))) (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 53 21) (end 53 32)) (probe (position 53 21))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 6 17) (end 6 26)) (probe (position 6 17))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint::wheelAssy"))) (kind featureTyping) (ordinal 0) (authored-target "WheelAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 14 21) (end 14 34)) (probe (position 14 21))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::discBrakeAssy"))) (kind featureTyping) (ordinal 0) (authored-target "DiscBrakeAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy")))))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 15 13) (end 15 18)) (probe (position 15 13))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 26 35) (end 26 54)) (probe (position 26 35))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::discBrakeConstraint"))) (kind featureTyping) (ordinal 0) (authored-target "DiscBrakeConstraint")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint")))))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 22 21) (end 22 32)) (probe (position 22 21))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind featureTyping) (ordinal 0) (authored-target "LengthValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 21 19) (end 21 28)) (probe (position 21 19))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 24 19) (end 24 28)) (probe (position 24 19))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind featureTyping) (ordinal 0) (authored-target "WheelAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 38 23) (end 38 36)) (probe (position 38 23))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind featureTyping) (ordinal 0) (authored-target "DiscBrakeAssy")
      (outcome (status resolved) (target (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy")))))
  )
  (query (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (range (start 37 15) (end 37 20)) (probe (position 37 15))
    (reference (id (source (node (document "memory://snapshot/15_05_unification_of_expression_and_constraint_definition.md") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind featureTyping) (ordinal 0) (authored-target "Wheel")
      (outcome (status unresolved)))
  )
)
~~~
