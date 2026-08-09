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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Verification Case Definition Example"))) (name "Verification Case Definition Example") (declared-name "Verification Case Definition Example")
      (contains
        (element (kind "part def") (id (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
          (contains
            (element (kind "attribute") (id (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle")))))
          )
        )
        (element (kind "verification def") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (name "VehicleMassTest") (declared-name "VehicleMassTest")
          (contains
            (element (kind "action") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData"))) (name "collectData") (declared-name "collectData") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (name "massMeasured") (declared-name "massMeasured") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (name "testVehicle") (declared-name "testVehicle") (declared (properties (direction "in") (composite true) (reference false) (ordered false)) (feature-value (kind bound) (expression (kind "featureReference") (reference "VehicleMassTest::testVehicle")))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (role feature-value))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData"))) (name "evaluateData") (declared-name "evaluateData") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (name "massProcessed") (declared-name "massProcessed") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (name "verdict") (declared-name "verdict") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData"))) (name "processData") (declared-name "processData") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
              (contains
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (name "massMeasured") (declared-name "massMeasured") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (name "massProcessed") (declared-name "massProcessed") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
              )
            )
            (element (kind "subject") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (name "testVehicle") (declared-name "testVehicle") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
            (element (kind "objective") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective"))) (name "vehicleMassVerificationObjective") (declared-name "vehicleMassVerificationObjective") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
              (contains
                (element (kind "verified requirement") (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest")))))
              )
            )
          )
        )
        (element (kind "requirement") (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::_documentation"))) (name ""))
            (element (kind "require constraint") (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0"))
            (element (kind "subject") (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (name "vehicle") (declared-name "vehicle"))
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (to (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
