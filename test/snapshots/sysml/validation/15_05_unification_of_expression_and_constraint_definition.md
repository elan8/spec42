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
  (document "15_05_unification_of_expression_and_constraint_definition.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 40))
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
        (range (start 3 16) (end 3 18))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 21 2) (end 21 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 2) (end 21 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 21 19) (end 21 28))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 22 2) (end 22 47))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 2) (end 22 47))
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
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 53 2) (end 53 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 46))
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
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a4157d352526353bbff0d814be3595151f203a68df37a47db3b16f9820c834df") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (kind "package") (name "15_05-Unification of Expression and Constraint Definition") (declared-name "15_05-Unification of Expression and Constraint Definition"))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (authored (membership (kind Import) (visibility "private") (import (reference "15_03-Value Expression::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (kind "part def") (name "DiscBrakeAssy") (declared-name "DiscBrakeAssy") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint"))) (kind "constraint def") (name "DiscBrakeConstraint") (declared-name "DiscBrakeConstraint") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt"))) (kind "constraint def") (name "DiscBrakeFitConstraint_Alt") (declared-name "DiscBrakeFitConstraint_Alt") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (kind "part def") (name "Vehicle_2") (declared-name "Vehicle_2") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind "attribute") (name "length") (declared-name "length") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue")) (typing (reference "LengthValue")))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelAssy")))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (kind "part def") (name "WheelAssy") (declared-name "WheelAssy") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind "part") (name "discBrakeAssy") (declared-name "discBrakeAssy") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiscBrakeAssy")))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel")))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "15_03-Value Expression::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelAssy") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "DiscBrakeAssy") (outcome (status resolved) (target (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (target (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (target (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 3 16) (end 3 18)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 3 16) (end 3 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 15) (end 37 20)) (probe (position 37 15))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))
        (kind featureTyping) (ordinal 0) (authored-target "Wheel")
        (range (start 37 15) (end 37 20))
        (outcome (status unresolved))
      )
    )
    (query (range (start 21 19) (end 21 28)) (probe (position 21 19))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 21 19) (end 21 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 24 19) (end 24 28)) (probe (position 24 19))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))
        (kind featureTyping) (ordinal 0) (authored-target "WheelAssy")
        (range (start 24 19) (end 24 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy") (range (start 36 1) (end 36 406)))
        )
      )
    )
    (query (range (start 22 21) (end 22 32)) (probe (position 22 21))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 22 21) (end 22 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 53 21) (end 53 32)) (probe (position 53 21))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))
        (kind featureTyping) (ordinal 1) (authored-target "LengthValue")
        (range (start 53 21) (end 53 32))
        (outcome (status unresolved))
      )
    )
    (query (range (start 38 23) (end 38 36)) (probe (position 38 23))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))
        (kind featureTyping) (ordinal 0) (authored-target "DiscBrakeAssy")
        (range (start 38 23) (end 38 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy") (range (start 52 1) (end 52 75)))
        )
      )
    )
    (query (range (start 1 16) (end 1 40)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "15_03-Value Expression::*")
        (range (start 1 16) (end 1 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 40)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::forAll"))
        (kind membershipImport) (ordinal 0) (authored-target "ControlFunctions::forAll")
        (range (start 2 16) (end 2 40))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
