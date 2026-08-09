# META
~~~ini
description=SysML Validation (09-Verification): 9-Verification-simplified
type=file
~~~
# SOURCE
~~~sysml
package '9-Verification-simplified' {
	private import VerificationCases::*;
	private import Definitions::*;
	
	package Definitions {
	
		requirement def <'2'> MassRequirement {
			attribute massActual :> ISQ::mass;
			attribute massReqd :> ISQ::mass;
			
			doc /* The actual mass shall be less than or equal to the required mass limit. */
			
			require constraint { massActual <= massReqd }
		}
		
		part def Vehicle {
			attribute mass :> ISQ::mass;
		}
		
		part def MassVerificationSystem;
		part def Scale;
		part def TestOperator;
		
		individual def TestVehicle1 :> Vehicle;
		individual def TestVehicle2 :> Vehicle;

		individual def TestSystem :> MassVerificationSystem;
	
		verification def MassTest {
			objective massVerificationObjective {
				verify requirement massRequirement : MassRequirement;
			}
		}
				
	}
	
	package Usages {
	
		requirement <'2.1'> vehicleMassRequirement : MassRequirement {
			subject vehicle : Vehicle;
			doc /* The vehicle mass shall be less than or equal to 2500 kg. */
			
			:>> massActual = vehicle.mass;		
			:>> massReqd = 2500 [SI::kg];
		}
		
		part vehicle1_c2 : Vehicle {
			// ...
		}
		
		verification vehicleMassTest : MassTest {
			subject testVehicle : Vehicle;
			objective vehicleMassVerificationObjective {
				// The subject of the verify is automatically bound to 'testVehicle' here.
				verify vehicleMassRequirement :>> massRequirement;
			}
			
			action collectData {
				in part testVehicle : Vehicle = vehicleMassTest.testVehicle;
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
					PassIf(vehicleMassRequirement(vehicle = new testVehicle(mass = massProcessed)));
			}
			
			return verdict : VerdictKind = evaluateData.verdict;
		}
		
		part massVerificationSystem : MassVerificationSystem {
			perform vehicleMassTest {
				in part :>> testVehicle = vehicleUnderTest;
			}
			
			ref part vehicleUnderTest : Vehicle;
			
			part testOperator : TestOperator;
			
			part scale : Scale {
				perform vehicleMassTest.collectData {
					in part :>> testVehicle;
					
					// In reality, this would be some more involved process.
					measurement = testVehicle.mass;
					
					out :>> massMeasured = measurement;
				}
			}
		}
		
		individual testSystem : TestSystem :> massVerificationSystem {
			timeslice test1 {
				ref individual :>> vehicleUnderTest : TestVehicle1 :> vehicle1_c2 {
					:>> mass = 2500 [SI::kg];
				}
			}
			
			then timeslice test2 {
				ref individual :>> vehicleUnderTest : TestVehicle2 :> vehicle1_c2 {
					:>> mass = 2500 [SI::kg];
				}
			}
		}
		
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwRequirement,KwDef,OpenAngle,UnrestrictedName,CloseAngle,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
KwDoc,RegularComment,
KwRequire,KwConstraint,OpenCurly,Ident,LtEq,Ident,CloseCurly,
CloseCurly,
KwPart,KwDef,Ident,OpenCurly,
KwAttribute,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwVerification,KwDef,Ident,OpenCurly,
KwObjective,Ident,OpenCurly,
KwVerify,KwRequirement,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwRequirement,OpenAngle,UnrestrictedName,CloseAngle,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwDoc,RegularComment,
ColonGtGt,Ident,Eq,Ident,Dot,Ident,Semicolon,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
LineComment,
CloseCurly,
KwVerification,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,OpenCurly,
LineComment,
KwVerify,Ident,ColonGtGt,Ident,Semicolon,
CloseCurly,
KwAction,Ident,OpenCurly,
KwIn,KwPart,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
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
Ident,OpenParen,Ident,OpenParen,Ident,Eq,Ident,Ident,OpenParen,Ident,Eq,Ident,CloseParen,CloseParen,CloseParen,Semicolon,
CloseCurly,
KwReturn,Ident,Colon,Ident,Eq,Ident,Dot,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,OpenCurly,
KwIn,KwPart,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
KwRef,KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,OpenCurly,
KwIn,KwPart,ColonGtGt,Ident,Semicolon,
LineComment,
Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwIndividual,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwTimeslice,Ident,OpenCurly,
KwRef,KwIndividual,ColonGtGt,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
KwThen,KwTimeslice,Ident,OpenCurly,
KwRef,KwIndividual,ColonGtGt,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''9-Verification-simplified''
    (import_decl private 'VerificationCases::*')
    (import_decl private 'Definitions::*')
    (package_def 'Definitions'
      (requirement_def 'MassRequirement'
        (attribute_usage 'massActual' :> 'ISQ::mass')
        (attribute_usage 'massReqd' :> 'ISQ::mass')
        (documentation)
        (sysml_decl
          (result_expr_member)))
      (part_def 'Vehicle'
        (attribute_usage 'mass' :> 'ISQ::mass'))
      (part_def 'MassVerificationSystem')
      (part_def 'Scale')
      (part_def 'TestOperator')
      (individual_def individual 'TestVehicle1' :> 'Vehicle')
      (individual_def individual 'TestVehicle2' :> 'Vehicle')
      (individual_def individual 'TestSystem' :> 'MassVerificationSystem')
      (verification_case_def 'MassTest'
        (objective_member)))
    (package_def 'Usages'
      (requirement_usage 'vehicleMassRequirement' : 'MassRequirement'
        (sysml_decl 'vehicle' : 'Vehicle')
        (documentation)
        (default_ref_usage :>> 'massActual' value)
        (default_ref_usage :>> 'massReqd' value))
      (part_usage 'vehicle1_c2' : 'Vehicle'
        (line_comment))
      (sysml_decl 'vehicleMassTest' : 'MassTest'
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
        (return_member))
      (part_usage 'massVerificationSystem' : 'MassVerificationSystem'
        (perform_action :>> 'vehicleMassTest'
          (part_usage in :>> 'testVehicle' value))
        (part_usage ref 'vehicleUnderTest' : 'Vehicle')
        (part_usage 'testOperator' : 'TestOperator')
        (part_usage 'scale' : 'Scale'
          (perform_action :>> 'vehicleMassTest.collectData'
            (part_usage in :>> 'testVehicle')
            (line_comment)
            (default_ref_usage 'measurement' value)
            (default_ref_usage out :>> 'massMeasured' value))))
      (individual_usage individual 'testSystem' : 'TestSystem' :> 'massVerificationSystem'
        (portion_usage timeslice 'test1'
          (individual_usage individual ref :>> 'vehicleUnderTest' : 'TestVehicle1' :> 'vehicle1_c2'
            (default_ref_usage :>> 'mass' value)))
        (source_succession
          (portion_usage timeslice 'test2'
            (individual_usage individual ref :>> 'vehicleUnderTest' : 'TestVehicle2' :> 'vehicle1_c2'
              (default_ref_usage :>> 'mass' value))))))))
~~~
# FORMAT
~~~sysml
package '9-Verification-simplified' {
    private import VerificationCases::*;
    private import Definitions::*;

    package Definitions {

        requirement def <'2'> MassRequirement {
            attribute massActual :> ISQ::mass;
            attribute massReqd :> ISQ::mass;

            doc /* The actual mass shall be less than or equal to the required mass limit. */

            require constraint { massActual <= massReqd }
        }

        part def Vehicle {
            attribute mass :> ISQ::mass;
        }

        part def MassVerificationSystem;
        part def Scale;
        part def TestOperator;

        individual def TestVehicle1 :> Vehicle;
        individual def TestVehicle2 :> Vehicle;

        individual def TestSystem :> MassVerificationSystem;

        verification def MassTest {
            objective massVerificationObjective {
                verify requirement massRequirement : MassRequirement;
            }
        }

    }

    package Usages {

        requirement <'2.1'> vehicleMassRequirement : MassRequirement {
            subject vehicle : Vehicle;
            doc /* The vehicle mass shall be less than or equal to 2500 kg. */

            :>> massActual = vehicle.mass;
            :>> massReqd = 2500 [SI::kg];
        }

        part vehicle1_c2 : Vehicle {
            // ...
        }

        verification vehicleMassTest : MassTest {
            subject testVehicle : Vehicle;
            objective vehicleMassVerificationObjective {
                // The subject of the verify is automatically bound to 'testVehicle' here.
                verify vehicleMassRequirement :>> massRequirement;
            }

            action collectData {
                in part testVehicle : Vehicle = vehicleMassTest.testVehicle;
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
                PassIf(vehicleMassRequirement(vehicle = new testVehicle(mass = massProcessed)));
            }

            return verdict : VerdictKind = evaluateData.verdict;
        }

        part massVerificationSystem : MassVerificationSystem {
            perform vehicleMassTest {
                in part :>> testVehicle = vehicleUnderTest;
            }

            ref part vehicleUnderTest : Vehicle;

            part testOperator : TestOperator;

            part scale : Scale {
                perform vehicleMassTest.collectData {
                    in part :>> testVehicle;

                    // In reality, this would be some more involved process.
                    measurement = testVehicle.mass;

                    out :>> massMeasured = measurement;
                }
            }
        }

        individual testSystem : TestSystem :> massVerificationSystem {
            timeslice test1 {
                ref individual :>> vehicleUnderTest : TestVehicle1 :> vehicle1_c2 {
                    :>> mass = 2500 [SI::kg];
                }
            }

            then timeslice test2 {
                ref individual :>> vehicleUnderTest : TestVehicle2 :> vehicle1_c2 {
                    :>> mass = 2500 [SI::kg];
                }
            }
        }

    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'massRequirement'
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
semantic.unresolved_name 'massRequirement'
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
    (element (kind "package") (id (node (document "d0") (qualified-name "9-Verification-simplified"))) (name "9-Verification-simplified") (declared-name "9-Verification-simplified")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "9-Verification-simplified::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "9-Verification-simplified::*#import"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (name "MassRequirement") (declared-name "MassRequirement")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
                (element (kind "require constraint") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_requireConstraint_0"))) (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
              )
            )
            (element (kind "verification def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (name "MassTest") (declared-name "MassTest")
              (contains
                (element (kind "objective") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective"))) (name "massVerificationObjective") (declared-name "massVerificationObjective") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
                  (contains
                    (element (kind "verified requirement") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (name "MassRequirement") (declared-name "MassRequirement") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                  )
                )
              )
            )
            (element (kind "part def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (name "MassVerificationSystem") (declared-name "MassVerificationSystem") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale"))) (name "Scale") (declared-name "Scale") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))) (name "TestOperator") (declared-name "TestOperator") (declared))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (name "TestSystem") (declared-name "TestSystem"))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (name "TestVehicle1") (declared-name "TestVehicle1"))
            (element (kind "individual def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (name "TestVehicle2") (declared-name "TestVehicle2"))
            (element (kind "part def") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared)
              (contains
                (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (composite true) (reference false) (ordered false) (unique true))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (name "massVerificationSystem") (declared-name "massVerificationSystem") (declared (properties (composite true) (reference false) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (name "scale") (declared-name "scale") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))))
                  (contains
                    (element (kind "action") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest.collectData"))) (name "vehicleMassTest.collectData") (declared-name "vehicleMassTest.collectData") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale")))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (name "testOperator") (declared-name "testOperator") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
                (element (kind "action") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest"))) (name "vehicleMassTest") (declared-name "vehicleMassTest") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
                (element (kind "ref") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (name "vehicleUnderTest") (declared-name "vehicleUnderTest") (declared (properties (composite true) (reference false) (ordered false))) (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
              )
            )
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (name "testSystem") (declared-name "testSystem") (declared (properties (individual true) (composite true) (reference false)))
              (contains
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1"))) (name "test1") (declared-name "test1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "timeslice")))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (name "") (declared (properties (individual true) (composite false) (reference true)))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (name "mass") (declared-name "mass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                  )
                )
                (element (kind "occurrence") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2"))) (name "test2") (declared-name "test2") (declared (properties (portion true) (composite true) (reference false) (portion-kind "timeslice")))
                  (contains
                    (element (kind "occurrence") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (name "") (declared (properties (individual true) (composite false) (reference true)))
                      (contains
                        (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (name "mass") (declared-name "mass") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                      )
                    )
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (name "vehicle1_c2") (declared-name "vehicle1_c2") (declared (properties (composite true) (reference false) (ordered false))))
            (element (kind "requirement") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (name "massActual") (declared-name "massActual") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (name "massReqd") (declared-name "massReqd") (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
              )
            )
            (element (kind "verification") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (name "vehicleMassTest") (declared-name "vehicleMassTest")
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (name "collectData") (declared-name "collectData") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::massMeasured"))) (name "massMeasured") (declared-name "massMeasured") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (name "testVehicle") (declared-name "testVehicle") (declared (properties (direction "in") (composite true) (reference false) (ordered false)) (feature-value (kind bound) (expression (kind "memberAccess") (reference "testVehicle") (children (expression (kind "featureReference") (reference "vehicleMassTest")))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (role feature-value))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (name "evaluateData") (declared-name "evaluateData") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::massProcessed"))) (name "massProcessed") (declared-name "massProcessed") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::verdict"))) (name "verdict") (declared-name "verdict") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                  )
                )
                (element (kind "action") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (name "processData") (declared-name "processData") (declared (properties (composite true) (reference false))) (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massMeasured"))) (name "massMeasured") (declared-name "massMeasured") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massProcessed"))) (name "massProcessed") (declared-name "massProcessed") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                  )
                )
                (element (kind "subject") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (name "testVehicle") (declared-name "testVehicle") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                (element (kind "objective") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective"))) (name "vehicleMassVerificationObjective") (declared-name "vehicleMassVerificationObjective") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
                  (contains
                    (element (kind "verified requirement") (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (effective (featuring-type (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
                  )
                )
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (annotation (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::_documentation"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))))
    (perform (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))))
    (redefinition (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (to (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "9-Verification-simplified::Usages::massVerificationSystem::scale") (target-qualified "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest::collectData"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/9_verification_simplified.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unresolved_pending_relationship")
        (source "semantic")
        (range (start 0 0) (end 0 0))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 37))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 23 2) (end 23 41))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 24 2) (end 24 41))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 26 2) (end 26 54))
      )
      (diagnostic
        (severity warning)
        (code "case_subject_missing")
        (source "semantic")
        (range (start 28 2) (end 28 137))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 29 3) (end 29 103))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 43 3) (end 43 32))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 101 5) (end 101 30))
      )
      (diagnostic
        (severity warning)
        (code "unknown_unit_symbol")
        (source "semantic")
        (range (start 107 5) (end 107 30))
      )
    )
  )
)
~~~
