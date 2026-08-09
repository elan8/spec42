# META
~~~ini
description=SysML Training 34 (Verification): Verification Case Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Verification Case Definition Example' {
	
	part def Vehicle {
		attribute mass :> ISQ::mass;
	}
	
	requirement vehicleMassRequirement {
		subject vehicle : Vehicle;
		in massActual :> ISQ::mass;
		doc /* The vehicle mass shall be less than or equal to 2500 kg. */
		
		require constraint { 
		    massActual == vehicle.mass and 
		    massActual <= 2500[SI::kg]
		}
	}
		
	verification def VehicleMassTest {
		private import VerificationCases::*;

		subject testVehicle : Vehicle;
		objective vehicleMassVerificationObjective {
			// The subject of the verify is automatically bound to 'testVehicle' here.
			verify vehicleMassRequirement;
		}
		
		action collectData {
			in part testVehicle : Vehicle = VehicleMassTest::testVehicle;
			out massMeasured :> ISQ::mass;
		}
		
		action processData {
			in massMeasured :> ISQ::mass = collectData.massMeasured;
			out massProcessed :> ISQ::mass;
		}
		
		action evaluateData {
			in massProcessed :> ISQ::mass = processData.massProcessed;
			out verdict : VerdictKind = 
				// Check that 'testVehicle' statisfies 'vehicleMassRequirement' if its mass equals 'massProcessed'.
				PassIf(vehicleMassRequirement(vehicle = testVehicle, massActual = massProcessed));
		}
		
		return verdict : VerdictKind = evaluateData.verdict;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwRequirement,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwDoc,RegularComment,
KwRequire,KwConstraint,OpenCurly,
Ident,EqEq,Ident,Dot,Ident,KwAnd,
Ident,LtEq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,
CloseCurly,
CloseCurly,
KwVerification,KwDef,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
LineComment,
KwVerify,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,KwPart,Ident,Colon,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
KwOut,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,Ident,ColonGt,Ident,ColonColon,Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,Ident,Colon,Ident,Eq,
LineComment,
Ident,OpenParen,Ident,OpenParen,Ident,Eq,Ident,Comma,Ident,Eq,Ident,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwReturn,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Verification Case Definition Example''
    (part_def 'Vehicle'
      (attribute_usage 'mass' :> 'ISQ::mass'))
    (requirement_usage 'vehicleMassRequirement'
      (sysml_decl 'vehicle' : 'Vehicle')
      (default_ref_usage in 'massActual' :> 'ISQ::mass')
      (documentation)
      (sysml_decl
        (result_expr_member)))
    (verification_case_def 'VehicleMassTest'
      (import_decl private 'VerificationCases::*')
      (sysml_decl 'testVehicle' : 'Vehicle')
      (objective_member)
      (action_usage 'collectData'
        (part_usage in 'testVehicle' : 'Vehicle' value)
        (default_ref_usage out 'massMeasured' :> 'ISQ::mass'))
      (action_usage 'processData'
        (default_ref_usage in 'massMeasured' :> 'ISQ::mass' value)
        (default_ref_usage out 'massProcessed' :> 'ISQ::mass'))
      (action_usage 'evaluateData'
        (default_ref_usage in 'massProcessed' :> 'ISQ::mass' value)
        (default_ref_usage out 'verdict' : 'VerdictKind' value))
      (return_member))))
~~~
# FORMAT
~~~sysml
package 'Verification Case Definition Example' {
    part def Vehicle {
        attribute mass :> ISQ::mass;
    }

    requirement vehicleMassRequirement {
        subject vehicle : Vehicle;
        in massActual :> ISQ::mass;
        doc /* The vehicle mass shall be less than or equal to 2500 kg. */

        require constraint {
            = massActual == vehicle.mass and massActual <= 2500[SI::kg];
        }
    }

    verification def VehicleMassTest {
        private import VerificationCases::*;

        subject testVehicle : Vehicle;
        objective vehicleMassVerificationObjective {
            // The subject of the verify is automatically bound to 'testVehicle' here.
            verify vehicleMassRequirement;
        }

        action collectData {
            in part testVehicle : Vehicle = VehicleMassTest::testVehicle;
            out massMeasured :> ISQ::mass;
        }

        action processData {
            in massMeasured :> ISQ::mass = collectData.massMeasured;
            out massProcessed :> ISQ::mass;
        }

        action evaluateData {
            in massProcessed :> ISQ::mass = processData.massProcessed;
            out verdict : VerdictKind = // Check that 'testVehicle' statisfies 'vehicleMassRequirement' if its mass equals 'massProcessed'.
				PassIf(vehicleMassRequirement(vehicle = testVehicle, massActual = massProcessed));
        }

        return verdict : VerdictKind = evaluateData.verdict;
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'VerdictKind'
semantic.unresolved_name 'VerdictKind'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'VerdictKind'
semantic.unresolved_name 'VerdictKind'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Verification Case Definition Example'
      (part_def 'Vehicle'
        (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
      (requirement_usage 'vehicleMassRequirement'
        (subject_membership in 'vehicle' : 'Verification Case Definition Example::Vehicle'[part_def])
        (reference_usage in reference 'massActual' :> 'ISQ::mass'[unresolved])
        (documentation)
        (require_constraint_usage composite
          (result_expr_membership)))
      (verification_case_def 'VehicleMassTest'
        (namespace_import private -> 'VerificationCases'[unresolved])
        (subject_membership in 'testVehicle' : 'Verification Case Definition Example::Vehicle'[part_def])
        (objective_membership composite 'vehicleMassVerificationObjective'
          (verify_requirement_membership 'vehicleMassRequirement'))
        (action_usage composite 'collectData'
          (part_usage in 'testVehicle' : 'Verification Case Definition Example::Vehicle'[part_def]
            (feature_value (=)))
          (reference_usage out reference 'massMeasured' :> 'ISQ::mass'[unresolved]))
        (action_usage composite 'processData'
          (reference_usage in reference 'massMeasured' :> 'ISQ::mass'[unresolved]
            (feature_value (=)))
          (reference_usage out reference 'massProcessed' :> 'ISQ::mass'[unresolved]))
        (action_usage composite 'evaluateData'
          (reference_usage in reference 'massProcessed' :> 'ISQ::mass'[unresolved]
            (feature_value (=)))
          (reference_usage out reference 'verdict' : 'VerdictKind'[unresolved]
            (feature_value (=))))
        (return_parameter_membership
          (feature_def out 'verdict' : 'VerdictKind'[unresolved]
            (feature_value (=))))))))
~~~
