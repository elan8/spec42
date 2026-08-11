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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "9_verification_simplified.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 7 27) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 8 25) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 16 21) (end 16 30))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 58 4) (end 58 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 59 4) (end 59 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 63 4) (end 63 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 64 4) (end 64 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 4) (end 68 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 4) (end 69 223))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 86 3) (end 86 258))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 100 23) (end 100 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 106 23) (end 106 39))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3ab05ac8c5711a50a444f7f5e7f3c00424af88e397cb8d412044e50a70b14248") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified"))) (kind "package") (name "9-Verification-simplified") (declared-name "9-Verification-simplified") (range (start (line 0) (character 0)) (end (line 0) (character 2911))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 37))) (parent (node (document "d0") (qualified-name "9-Verification-simplified"))) (authored (membership (kind Import) (visibility "private") (import (reference "VerificationCases::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 33))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 31))) (parent (node (document "d0") (qualified-name "9-Verification-simplified"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 27))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 4) (character 1)) (end (line 4) (character 718))) (parent (node (document "d0") (qualified-name "9-Verification-simplified"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (kind "requirement def") (name "MassRequirement") (declared-name "MassRequirement") (range (start (line 6) (character 2)) (end (line 6) (character 261))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 6) (character 2)) (end (line 6) (character 261))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (range (start (line 12) (character 3)) (end (line 12) (character 48))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 7) (character 3)) (end (line 7) (character 37))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (authored (relationships (subsetting (reference "ISQ::mass") (range (start (line 7) (character 27)) (end (line 7) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 8) (character 3)) (end (line 8) (character 35))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (authored (relationships (subsetting (reference "ISQ::mass") (range (start (line 8) (character 25)) (end (line 8) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (kind "verification def") (name "MassTest") (declared-name "MassTest") (range (start (line 28) (character 2)) (end (line 28) (character 137))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective"))) (kind "objective") (name "massVerificationObjective") (declared-name "massVerificationObjective") (range (start (line 29) (character 3)) (end (line 29) (character 103))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind "verified requirement") (name "MassRequirement") (declared-name "MassRequirement") (range (start (line 30) (character 4)) (end (line 30) (character 57))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective"))) (authored (relationships (typing (reference "MassRequirement") (range none)) (subject (reference "MassRequirement") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (kind "part def") (name "MassVerificationSystem") (declared-name "MassVerificationSystem") (range (start (line 19) (character 2)) (end (line 19) (character 34))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale"))) (kind "part def") (name "Scale") (declared-name "Scale") (range (start (line 20) (character 2)) (end (line 20) (character 17))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))) (kind "part def") (name "TestOperator") (declared-name "TestOperator") (range (start (line 21) (character 2)) (end (line 21) (character 24))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (kind "individual def") (name "TestSystem") (declared-name "TestSystem") (range (start (line 26) (character 2)) (end (line 26) (character 54))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassVerificationSystem") (range (start (line 26) (character 31)) (end (line 26) (character 53)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (kind "individual def") (name "TestVehicle1") (declared-name "TestVehicle1") (range (start (line 23) (character 2)) (end (line 23) (character 41))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 23) (character 33)) (end (line 23) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (kind "individual def") (name "TestVehicle2") (declared-name "TestVehicle2") (range (start (line 24) (character 2)) (end (line 24) (character 41))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 24) (character 33)) (end (line 24) (character 40)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 15) (character 2)) (end (line 15) (character 56))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 16) (character 3)) (end (line 16) (character 31))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass") (range (start (line 16) (character 21)) (end (line 16) (character 30)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 36) (character 1)) (end (line 36) (character 2076))) (parent (node (document "d0") (qualified-name "9-Verification-simplified"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind "part") (name "massVerificationSystem") (declared-name "massVerificationSystem") (range (start (line 77) (character 2)) (end (line 77) (character 490))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassVerificationSystem") (range (start (line 77) (character 32)) (end (line 77) (character 54)))) (perform (reference "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind "part") (name "scale") (declared-name "scale") (range (start (line 86) (character 3)) (end (line 86) (character 258))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scale") (range (start (line 86) (character 16)) (end (line 86) (character 21)))) (perform (reference "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest::collectData") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest.collectData"))) (kind "action") (name "vehicleMassTest.collectData") (declared-name "vehicleMassTest.collectData") (range (start (line 87) (character 4)) (end (line 87) (character 229))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind "part") (name "testOperator") (declared-name "testOperator") (range (start (line 84) (character 3)) (end (line 84) (character 36))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestOperator") (range (start (line 84) (character 23)) (end (line 84) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest"))) (kind "action") (name "vehicleMassTest") (declared-name "vehicleMassTest") (range (start (line 78) (character 3)) (end (line 78) (character 81))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind "ref") (name "vehicleUnderTest") (declared-name "vehicleUnderTest") (range (start (line 82) (character 3)) (end (line 82) (character 39))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 82) (character 31)) (end (line 82) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind "occurrence") (name "testSystem") (declared-name "testSystem") (range (start (line 98) (character 13)) (end (line 98) (character 347))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestSystem") (range none)) (subsetting (reference "massVerificationSystem") (range (start (line 98) (character 40)) (end (line 98) (character 62)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1"))) (kind "occurrence") (name "test1") (declared-name "test1") (range (start (line 99) (character 13)) (end (line 99) (character 134))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind "occurrence") (name "") (range (start (line 100) (character 19)) (end (line 100) (character 108))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestVehicle1") (range none)) (subsetting (reference "vehicle1_c2") (range (start (line 100) (character 58)) (end (line 100) (character 69)))) (redefinition (reference "vehicleUnderTest") (range (start (line 100) (character 23)) (end (line 100) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 101) (character 5)) (end (line 101) (character 30))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 101) (character 5)) (end (line 101) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2"))) (kind "occurrence") (name "test2") (declared-name "test2") (range (start (line 105) (character 18)) (end (line 105) (character 139))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind "occurrence") (name "") (range (start (line 106) (character 19)) (end (line 106) (character 108))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestVehicle2") (range none)) (subsetting (reference "vehicle1_c2") (range (start (line 106) (character 58)) (end (line 106) (character 69)))) (redefinition (reference "vehicleUnderTest") (range (start (line 106) (character 23)) (end (line 106) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 107) (character 5)) (end (line 107) (character 30))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass") (range (start (line 107) (character 5)) (end (line 107) (character 13)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind "part") (name "vehicle1_c2") (declared-name "vehicle1_c2") (range (start (line 46) (character 2)) (end (line 46) (character 44))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 46) (character 21)) (end (line 46) (character 28)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (range (start (line 38) (character 2)) (end (line 38) (character 241))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement") (range none)) (subject (reference "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::_documentation"))) (kind "documentation") (name "") (range (start (line 38) (character 2)) (end (line 38) (character 241))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (range (start (line 42) (character 3)) (end (line 42) (character 33))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (authored (relationships (redefinition (reference "massActual") (range (start (line 42) (character 3)) (end (line 42) (character 17)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (range (start (line 43) (character 3)) (end (line 43) (character 32))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (authored (relationships (redefinition (reference "massReqd") (range (start (line 43) (character 3)) (end (line 43) (character 15)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 39) (character 3)) (end (line 39) (character 29))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (kind "verification") (name "vehicleMassTest") (declared-name "vehicleMassTest") (range (start (line 50) (character 2)) (end (line 50) (character 912))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassTest") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (kind "action") (name "collectData") (declared-name "collectData") (range (start (line 57) (character 3)) (end (line 57) (character 128))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::massMeasured"))) (kind "in out parameter") (name "massMeasured") (declared-name "massMeasured") (range (start (line 59) (character 4)) (end (line 59) (character 34))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (authored (relationships (typing (reference "ISQ::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (kind "part") (name "testVehicle") (declared-name "testVehicle") (range (start (line 58) (character 4)) (end (line 58) (character 64))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 58) (character 26)) (end (line 58) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (kind "action") (name "evaluateData") (declared-name "evaluateData") (range (start (line 67) (character 3)) (end (line 67) (character 316))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::massProcessed"))) (kind "in out parameter") (name "massProcessed") (declared-name "massProcessed") (range (start (line 68) (character 4)) (end (line 68) (character 62))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (authored (relationships (typing (reference "ISQ::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::verdict"))) (kind "in out parameter") (name "verdict") (declared-name "verdict") (range (start (line 69) (character 4)) (end (line 69) (character 223))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (authored (relationships (typing (reference "VerdictKind") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (kind "action") (name "processData") (declared-name "processData") (range (start (line 62) (character 3)) (end (line 62) (character 125))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massMeasured"))) (kind "in out parameter") (name "massMeasured") (declared-name "massMeasured") (range (start (line 63) (character 4)) (end (line 63) (character 60))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (authored (relationships (typing (reference "ISQ::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massProcessed"))) (kind "in out parameter") (name "massProcessed") (declared-name "massProcessed") (range (start (line 64) (character 4)) (end (line 64) (character 35))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (authored (relationships (typing (reference "ISQ::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (kind "subject") (name "testVehicle") (declared-name "testVehicle") (range (start (line 51) (character 3)) (end (line 51) (character 33))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective"))) (kind "objective") (name "vehicleMassVerificationObjective") (declared-name "vehicleMassVerificationObjective") (range (start (line 52) (character 3)) (end (line 52) (character 186))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind "verified requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (range (start (line 54) (character 4)) (end (line 54) (character 54))) (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective"))) (authored (relationships (typing (reference "vehicleMassRequirement") (range none)) (subject (reference "vehicleMassRequirement") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VerificationCases::*") (range (start (line 1) (character 16)) (end (line 1) (character 33))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 2) (character 16)) (end (line 2) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 7) (character 27)) (end (line 7) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 8) (character 25)) (end (line 8) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "MassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (kind specialization) (ordinal 0)) (authored-target "MassVerificationSystem") (range (start (line 26) (character 31)) (end (line 26) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 23) (character 33)) (end (line 23) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 24) (character 33)) (end (line 24) (character 40))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (range (start (line 16) (character 21)) (end (line 16) (character 30))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "MassVerificationSystem") (range (start (line 77) (character 32)) (end (line 77) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind performSource) (ordinal 0)) (authored-target "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)) (authored-target "Scale") (range (start (line 86) (character 16)) (end (line 86) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind performSource) (ordinal 0)) (authored-target "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest::collectData") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind featureTyping) (ordinal 0)) (authored-target "TestOperator") (range (start (line 84) (character 23)) (end (line 84) (character 35))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 82) (character 31)) (end (line 82) (character 38))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "TestSystem") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind subsetting) (ordinal 0)) (authored-target "massVerificationSystem") (range (start (line 98) (character 40)) (end (line 98) (character 62))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind featureTyping) (ordinal 0)) (authored-target "TestVehicle1") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c2") (range (start (line 100) (character 58)) (end (line 100) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind redefinition) (ordinal 0)) (authored-target "vehicleUnderTest") (range (start (line 100) (character 23)) (end (line 100) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 101) (character 5)) (end (line 101) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind featureTyping) (ordinal 0)) (authored-target "TestVehicle2") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c2") (range (start (line 106) (character 58)) (end (line 106) (character 69))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind redefinition) (ordinal 0)) (authored-target "vehicleUnderTest") (range (start (line 106) (character 23)) (end (line 106) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 107) (character 5)) (end (line 107) (character 13))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 46) (character 21)) (end (line 46) (character 28))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (range (start (line 42) (character 3)) (end (line 42) (character 17))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (range (start (line 43) (character 3)) (end (line 43) (character 15))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (kind featureTyping) (ordinal 0)) (authored-target "MassTest") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::massMeasured"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 58) (character 26)) (end (line 58) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::massProcessed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0)) (authored-target "VerdictKind") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "vehicleMassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "vehicleMassRequirement") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind subsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_requireConstraint_0")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement")) (expression (status "incomplete") (error "expression is incomplete")) (analysis (status "incomplete")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::massProcessed")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::verdict")) (expression (status "unsupported") (error "declared expression form is not supported")))
    (node (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massMeasured")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
