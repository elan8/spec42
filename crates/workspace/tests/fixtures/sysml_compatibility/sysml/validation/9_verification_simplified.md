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

            require constraint {
                = massActual <= massReqd;
            }
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
                verify massRequirement : MassRequirement;
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
                out verdict : VerdictKind = // Check that 'testVehicle' statisfies 'vehicleMassRequirement' if its mass equals 'massProcessed'.
					PassIf(vehicleMassRequirement(vehicle = new testVehicle(mass = massProcessed)));
            }

            return verdict : VerdictKind = evaluateData.verdict;
        }

        part massVerificationSystem : MassVerificationSystem {
            perform :>> vehicleMassTest {
                in part :>> testVehicle = vehicleUnderTest;
            }

            ref part vehicleUnderTest : Vehicle;

            part testOperator : TestOperator;

            part scale : Scale {
                perform :>> vehicleMassTest.collectData {
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
(model
  (namespace
    (package '9-Verification-simplified'
      (namespace_import private -> 'VerificationCases'[unresolved])
      (namespace_import private -> '9-Verification-simplified::Definitions'[package])
      (package 'Definitions'
        (requirement_def 'MassRequirement'
          (attribute_usage composite 'massActual' :> 'ISQ::mass'[unresolved])
          (attribute_usage composite 'massReqd' :> 'ISQ::mass'[unresolved])
          (documentation)
          (require_constraint_usage composite
            (result_expr_membership)))
        (part_def 'Vehicle'
          (attribute_usage composite 'mass' :> 'ISQ::mass'[unresolved]))
        (part_def 'MassVerificationSystem')
        (part_def 'Scale')
        (part_def 'TestOperator')
        (occurrence_def individual 'TestVehicle1' :> '9-Verification-simplified::Definitions::Vehicle'[part_def])
        (occurrence_def individual 'TestVehicle2' :> '9-Verification-simplified::Definitions::Vehicle'[part_def])
        (occurrence_def individual 'TestSystem' :> '9-Verification-simplified::Definitions::MassVerificationSystem'[part_def])
        (verification_case_def 'MassTest'
          (objective_membership composite 'massVerificationObjective'
            (verify_requirement_membership 'massRequirement' : '9-Verification-simplified::Definitions::MassRequirement'[requirement_def]))))
      (package 'Usages'
        (requirement_usage 'vehicleMassRequirement' : '9-Verification-simplified::Definitions::MassRequirement'[requirement_def]
          (subject_membership in 'vehicle' : '9-Verification-simplified::Definitions::Vehicle'[part_def])
          (documentation)
          (reference_usage reference :>> '9-Verification-simplified::Definitions::MassRequirement::massActual'[attribute_usage]
            (feature_value (=)))
          (reference_usage reference :>> '9-Verification-simplified::Definitions::MassRequirement::massReqd'[attribute_usage]
            (feature_value (=))))
        (part_usage 'vehicle1_c2' : '9-Verification-simplified::Definitions::Vehicle'[part_def])
        (verification_case_usage 'vehicleMassTest' : '9-Verification-simplified::Definitions::MassTest'[verification_case_def]
          (subject_membership in 'testVehicle' : '9-Verification-simplified::Definitions::Vehicle'[part_def])
          (objective_membership composite 'vehicleMassVerificationObjective'
            (verify_requirement_membership 'vehicleMassRequirement' :>> 'massRequirement'[unresolved]))
          (action_usage composite 'collectData'
            (part_usage in 'testVehicle' : '9-Verification-simplified::Definitions::Vehicle'[part_def]
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
              (feature_value (=)))))
        (part_usage 'massVerificationSystem' : '9-Verification-simplified::Definitions::MassVerificationSystem'[part_def]
          (perform_action_usage :>> '9-Verification-simplified::Usages::vehicleMassTest'[verification_case_usage]
            (part_usage in :>> '9-Verification-simplified::Usages::vehicleMassTest::testVehicle'[subject_membership]
              (feature_value (=))))
          (part_usage reference 'vehicleUnderTest' : '9-Verification-simplified::Definitions::Vehicle'[part_def])
          (part_usage composite 'testOperator' : '9-Verification-simplified::Definitions::TestOperator'[part_def])
          (part_usage composite 'scale' : '9-Verification-simplified::Definitions::Scale'[part_def]
            (perform_action_usage :>> '9-Verification-simplified::Usages::vehicleMassTest::collectData'[action_usage]
              (part_usage in :>> '9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle'[part_usage])
              (reference_usage reference 'measurement'
                (feature_value (=)))
              (reference_usage out reference :>> '9-Verification-simplified::Usages::vehicleMassTest::collectData::massMeasured'[reference_usage]
                (feature_value (=))))))
        (occurrence_usage individual 'testSystem' : '9-Verification-simplified::Definitions::TestSystem'[occurrence_def] :> '9-Verification-simplified::Usages::massVerificationSystem'[part_usage]
          (occurrence_usage composite 'test1'
            (occurrence_usage individual reference :>> '9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest'[part_usage] : '9-Verification-simplified::Definitions::TestVehicle1'[occurrence_def] :> '9-Verification-simplified::Usages::vehicle1_c2'[part_usage]
              (reference_usage reference :>> '9-Verification-simplified::Definitions::Vehicle::mass'[attribute_usage]
                (feature_value (=)))))
          (source_succession
            (occurrence_usage 'test2'
              (occurrence_usage individual reference :>> '9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest'[part_usage] : '9-Verification-simplified::Definitions::TestVehicle2'[occurrence_def] :> '9-Verification-simplified::Usages::vehicle1_c2'[part_usage]
                (reference_usage reference :>> '9-Verification-simplified::Definitions::Vehicle::mass'[attribute_usage]
                  (feature_value (=)))))))))))
~~~
