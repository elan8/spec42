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
        in wheelAssy : WheelAssy [4];

        = wheelAssy->forAll {in ref w: WheelAssy; 
			2 * w.discBrakeAssy.radius < w.wheel.outerDiameter
		};
    }

    constraint def DiscBrakeFitConstraint_Alt {
        in discBrakeAssy : DiscBrakeAssy [1];
        in wheel : Wheel [1];

        = 2 * discBrakeAssy.radius < wheel.outerDiameter;
    }

    part def Vehicle_2 {
        attribute mass : MassValue [1] = 1200 [kg];
        attribute length : LengthValue [1] = 4.82 [m];

        part wheelAssy : WheelAssy [4];

        constraint discBrakeConstraint : DiscBrakeConstraint {
            doc /*
			 * This constraint is computed, but not asserted. This means a tool can identify 
			 * when it is violated without the model being inconsistent.
			 */
            in wheelAssy = Vehicle_2::wheelAssy;
        }
    }

    part def WheelAssy {
        part wheel : Wheel [1];
        part discBrakeAssy : DiscBrakeAssy [1];

        assert constraint discBrakeFitConstraint_Alt : DiscBrakeFitConstraint_Alt {
            doc /*
			 * This constraint is asserted to be true, which means that the model
			 * is inconsistent if it the constraint is violated.
			 */

            in discBrakeAssy = WheelAssy::discBrakeAssy;
            in wheel = WheelAssy::wheel;
        }
    }

    part def DiscBrakeAssy {
        attribute radius : LengthValue [1] = 95 [mm];
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
(model
  (namespace
    (package '15_05-Unification of Expression and Constraint Definition'
      (namespace_import private -> '15_03-Value Expression'[unresolved])
      (membership_import private -> 'ControlFunctions::forAll'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (constraint_def 'DiscBrakeConstraint'
        (reference_usage in reference 'wheelAssy' : '15_05-Unification of Expression and Constraint Definition::WheelAssy'[part_def]
          (multiplicity_range [4]))
        (result_expr_membership))
      (constraint_def 'DiscBrakeFitConstraint_Alt'
        (reference_usage in reference 'discBrakeAssy' : '15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy'[part_def]
          (multiplicity_range [1]))
        (reference_usage in reference 'wheel' : 'Wheel'[unresolved]
          (multiplicity_range [1]))
        (result_expr_membership))
      (part_def 'Vehicle_2'
        (attribute_usage composite 'mass' : 'MassValue'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (attribute_usage composite 'length' : 'LengthValue'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))
        (part_usage composite 'wheelAssy' : '15_05-Unification of Expression and Constraint Definition::WheelAssy'[part_def]
          (multiplicity_range [4]))
        (constraint_usage composite 'discBrakeConstraint' : '15_05-Unification of Expression and Constraint Definition::DiscBrakeConstraint'[constraint_def]
          (documentation)
          (reference_usage in reference 'wheelAssy'
            (feature_value (=)))))
      (part_def 'WheelAssy'
        (part_usage composite 'wheel' : 'Wheel'[unresolved]
          (multiplicity_range [1]))
        (part_usage composite 'discBrakeAssy' : '15_05-Unification of Expression and Constraint Definition::DiscBrakeAssy'[part_def]
          (multiplicity_range [1]))
        (assert_constraint_usage 'discBrakeFitConstraint_Alt' : '15_05-Unification of Expression and Constraint Definition::DiscBrakeFitConstraint_Alt'[constraint_def]
          (documentation)
          (reference_usage in reference 'discBrakeAssy'
            (feature_value (=)))
          (reference_usage in reference 'wheel'
            (feature_value (=)))))
      (part_def 'DiscBrakeAssy'
        (attribute_usage composite 'radius' : 'LengthValue'[unresolved]
          (multiplicity_range [1])
          (feature_value (=)))))))
~~~
