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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "a4157d352526353bbff0d814be3595151f203a68df37a47db3b16f9820c834df") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (kind "package") (name "15_05-Unification of Expression and Constraint Definition") (declared-name "15_05-Unification of Expression and Constraint Definition") (range (start (line 0) (character 0)) (end (line 0) (character 1441))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 44))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (authored (membership (kind Import) (visibility "private") (import (reference "15_03-Value Expression::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 40))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 22))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 18))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (kind "part def") (name "DiscBrakeAssy") (declared-name "DiscBrakeAssy") (range (start (line 52) (character 1)) (end (line 52) (character 75))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind "attribute") (name "radius") (declared-name "radius") (range (start (line 53) (character 2)) (end (line 53) (character 46))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 53) (character 21)) (end (line 53) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint"))) (kind "constraint def") (name "DiscBrakeConstraint") (declared-name "DiscBrakeConstraint") (range (start (line 5) (character 1)) (end (line 5) (character 175))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt"))) (kind "constraint def") (name "DiscBrakeFitConstraint_Alt") (declared-name "DiscBrakeFitConstraint_Alt") (range (start (line 13) (character 1)) (end (line 13) (character 163))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (kind "part def") (name "Vehicle_2") (declared-name "Vehicle_2") (range (start (line 20) (character 1)) (end (line 20) (character 426))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind "attribute") (name "length") (declared-name "length") (range (start (line 22) (character 2)) (end (line 22) (character 47))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "LengthValue") (range none)) (typing (reference "LengthValue") (range (start (line 22) (character 21)) (end (line 22) (character 32)))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 21) (character 2)) (end (line 21) (character 44))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 21) (character 19)) (end (line 21) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind "part") (name "wheelAssy") (declared-name "wheelAssy") (range (start (line 24) (character 2)) (end (line 24) (character 32))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (authored (membership (kind Feature)) (relationships (typing (reference "WheelAssy") (range (start (line 24) (character 19)) (end (line 24) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (kind "part def") (name "WheelAssy") (declared-name "WheelAssy") (range (start (line 36) (character 1)) (end (line 36) (character 406))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind "part") (name "discBrakeAssy") (declared-name "discBrakeAssy") (range (start (line 38) (character 2)) (end (line 38) (character 40))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (authored (membership (kind Feature)) (relationships (typing (reference "DiscBrakeAssy") (range (start (line 38) (character 23)) (end (line 38) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind "part") (name "wheel") (declared-name "wheel") (range (start (line 37) (character 2)) (end (line 37) (character 24))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (authored (membership (kind Feature)) (relationships (typing (reference "Wheel") (range (start (line 37) (character 15)) (end (line 37) (character 20)))))))
    (element (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::forAll"))) (kind "import") (name "forAll") (declared-name "forAll") (range (start (line 2) (character 1)) (end (line 2) (character 41))) (parent (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (authored (membership (kind Import) (visibility "private") (import (reference "ControlFunctions::forAll") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 40))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "15_03-Value Expression::*") (range (start (line 1) (character 16)) (end (line 1) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 3) (character 16)) (end (line 3) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 53) (character 21)) (end (line 53) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind featureTyping) (ordinal 0)) (authored-target "LengthValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (kind featureTyping) (ordinal 1)) (authored-target "LengthValue") (range (start (line 22) (character 21)) (end (line 22) (character 32))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 21) (character 19)) (end (line 21) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "WheelAssy") (range (start (line 24) (character 19)) (end (line 24) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (kind featureTyping) (ordinal 0)) (authored-target "DiscBrakeAssy") (range (start (line 38) (character 23)) (end (line 38) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy")))))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (kind featureTyping) (ordinal 0)) (authored-target "Wheel") (range (start (line 37) (character 15)) (end (line 37) (character 20))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::forAll"))) (kind membershipImport) (ordinal 0)) (authored-target "ControlFunctions::forAll") (range (start (line 2) (character 16)) (end (line 2) (character 40))) (outcome (status unresolved)))
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
