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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
Ident,Arrow,Ident,OpenCurly,KwIn,KwRef,Ident,Colon,Ident,Semicolon,
DecimalValue,Star,Ident,Dot,Ident,Dot,Ident,OpenAngle,Ident,Dot,Ident,Dot,Ident,
CloseCurly,
CloseCurly,
KwConstraint,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwIn,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
DecimalValue,Star,Ident,Dot,Ident,OpenAngle,Ident,Dot,Ident,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,DecimalValue,Dot,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwConstraint,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwAssert,KwConstraint,Ident,Colon,Ident,OpenCurly,
KwDoc,
RegularComment,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,Colon,Ident,OpenSquare,DecimalValue,CloseSquare,Eq,DecimalValue,OpenSquare,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''15_05-Unification of Expression and Constraint Definition''
    (import_decl private ''15_03-Value Expression'::*')
    (import_decl private 'ControlFunctions::forAll')
    (import_decl private 'SI::*')
    (constraint_def 'DiscBrakeConstraint'
      (default_ref_usage in 'wheelAssy' : 'WheelAssy' multiplicity)
      (result_expr_member))
    (constraint_def 'DiscBrakeFitConstraint_Alt'
      (default_ref_usage in 'discBrakeAssy' : 'DiscBrakeAssy' multiplicity)
      (default_ref_usage in 'wheel' : 'Wheel' multiplicity)
      (result_expr_member))
    (part_def 'Vehicle_2'
      (attribute_usage 'mass' : 'MassValue' multiplicity value)
      (attribute_usage 'length' : 'LengthValue' multiplicity value)
      (part_usage 'wheelAssy' : 'WheelAssy' multiplicity)
      (constraint_usage 'discBrakeConstraint' : 'DiscBrakeConstraint'
        (documentation)
        (default_ref_usage in 'wheelAssy' value)))
    (part_def 'WheelAssy'
      (part_usage 'wheel' : 'Wheel' multiplicity)
      (part_usage 'discBrakeAssy' : 'DiscBrakeAssy' multiplicity)
      (sysml_decl 'discBrakeFitConstraint_Alt' : 'DiscBrakeFitConstraint_Alt'
        (documentation)
        (default_ref_usage in 'discBrakeAssy' value)
        (default_ref_usage in 'wheel' value)))
    (part_def 'DiscBrakeAssy'
      (attribute_usage 'radius' : 'LengthValue' multiplicity value))))
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
# EXPECTED
~~~
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'LengthValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'LengthValue'
semantic.unresolved_name 'Wheel'
semantic.unresolved_name 'LengthValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition"))) (name "15_05-Unification of Expression and Constraint Definition") (declared-name "15_05-Unification of Expression and Constraint Definition")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::*#import"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (name "DiscBrakeAssy") (declared-name "DiscBrakeAssy") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (name "radius") (declared-name "radius") (declared (properties (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 95))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "mm")))))))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
          )
        )
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint"))) (name "DiscBrakeConstraint") (declared-name "DiscBrakeConstraint") (declared (own-expression (expression (kind "collectionOperation") (operator "forAll") (children (expression (kind "featureReference") (reference "wheelAssy")))))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
        (element (kind "constraint def") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt"))) (name "DiscBrakeFitConstraint_Alt") (declared-name "DiscBrakeFitConstraint_Alt") (declared (own-expression (expression (kind "binary") (operator "<") (children (expression (kind "binary") (operator "*") (children (expression (kind "integerLiteral") (literal (integer 2))) (expression (kind "memberAccess") (reference "radius") (children (expression (kind "featureReference") (reference "discBrakeAssy")))))) (expression (kind "memberAccess") (reference "outerDiameter") (children (expression (kind "featureReference") (reference "wheel")))))))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (name "Vehicle_2") (declared-name "Vehicle_2") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (name "length") (declared-name "length") (declared (properties (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "realLiteral") (literal (real "4.82"))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "m")))))))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
            (element (kind "attribute") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored)) (feature-value (kind bound) (expression (kind "literalWithUnit") (children (expression (kind "integerLiteral") (literal (integer 1200))) (expression (kind "bracket") (children (expression (kind "featureReference") (reference "kg")))))))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
            (element (kind "part") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (name "wheelAssy") (declared-name "wheelAssy") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (name "WheelAssy") (declared-name "WheelAssy") (declared)
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (name "discBrakeAssy") (declared-name "discBrakeAssy") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
            (element (kind "part") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (name "wheel") (declared-name "wheel") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::forAll"))) (name "forAll") (declared-name "forAll"))
      )
    )
  )
  (relationships
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (to (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (to (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy::radius"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt"))) (status missing-prerequisite) (target "Constraints::ConstraintCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::length"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::Vehicle_2::wheelAssy"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::discBrakeAssy"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "15_05-Unification of Expression and Constraint Definition::WheelAssy::wheel"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/15_05_unification_of_expression_and_constraint_definition.md"
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
        (severity warning)
        (code "unknown_unit_symbol")
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
        (code "unknown_unit_symbol")
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
        (range (start 37 15) (end 37 20))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 53 2) (end 53 46))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 53 2) (end 53 46))
      )
    )
  )
)
~~~
