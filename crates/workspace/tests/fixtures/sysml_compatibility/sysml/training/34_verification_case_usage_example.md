# META
~~~ini
description=SysML Training 34 (Verification): Verification Case Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Verification Case Usage Example' {
	private import 'Verification Case Definition Example'::*;
	
	part def MassVerificationSystem;
	part def Scale;
	
	part vehicleTestConfig : Vehicle {
		// ...
	}
	
	verification vehicleMassTest : VehicleMassTest {
		subject testVehicle :> vehicleTestConfig;
	}
	
	part massVerificationSystem : MassVerificationSystem {
		perform vehicleMassTest;
		
		part scale : Scale {
			perform vehicleMassTest.collectData {
				in part :>> testVehicle;
				
				// In reality, this would be some more involved process.
				measurement = testVehicle.mass;
				
				out :>> massMeasured = measurement;
			}
		}
	}		
		
	individual def TestSystem :> MassVerificationSystem;
	
	individual def TestVehicle1 :> Vehicle;
	individual def TestVehicle2 :> Vehicle;

	individual testSystem : TestSystem :> massVerificationSystem {
		timeslice test1 {
			perform action :>> vehicleMassTest {
				in individual :>> testVehicle : TestVehicle1 {
					:>> mass = 2500[SI::kg];
				}
			}
		}
		
		then timeslice test2 {
			perform action :>> vehicleMassTest {
				in individual :>> testVehicle : TestVehicle2 {
					:>> mass = 3000[SI::kg];
				}
			}
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,UnrestrictedName,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
LineComment,
CloseCurly,
KwVerification,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,ColonGt,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Semicolon,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPerform,Ident,Dot,Ident,OpenCurly,
KwIn,KwPart,ColonGtGt,Ident,Semicolon,
LineComment,
Ident,Eq,Ident,Dot,Ident,Semicolon,
KwOut,ColonGtGt,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,KwDef,Ident,ColonGt,Ident,Semicolon,
KwIndividual,Ident,Colon,Ident,ColonGt,Ident,OpenCurly,
KwTimeslice,Ident,OpenCurly,
KwPerform,KwAction,ColonGtGt,Ident,OpenCurly,
KwIn,KwIndividual,ColonGtGt,Ident,Colon,Ident,OpenCurly,
ColonGtGt,Ident,Eq,DecimalValue,OpenSquare,Ident,ColonColon,Ident,CloseSquare,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwThen,KwTimeslice,Ident,OpenCurly,
KwPerform,KwAction,ColonGtGt,Ident,OpenCurly,
KwIn,KwIndividual,ColonGtGt,Ident,Colon,Ident,OpenCurly,
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
  (package_def ''Verification Case Usage Example''
    (import_decl private ''Verification Case Definition Example'::*')
    (part_def 'MassVerificationSystem')
    (part_def 'Scale')
    (part_usage 'vehicleTestConfig' : 'Vehicle'
      (line_comment))
    (sysml_decl 'vehicleMassTest' : 'VehicleMassTest'
      (sysml_decl 'testVehicle' :> 'vehicleTestConfig'))
    (part_usage 'massVerificationSystem' : 'MassVerificationSystem'
      (perform_action :>> 'vehicleMassTest')
      (part_usage 'scale' : 'Scale'
        (perform_action :>> 'vehicleMassTest.collectData'
          (part_usage in :>> 'testVehicle')
          (line_comment)
          (default_ref_usage 'measurement' value)
          (default_ref_usage out :>> 'massMeasured' value))))
    (individual_def individual 'TestSystem' :> 'MassVerificationSystem')
    (individual_def individual 'TestVehicle1' :> 'Vehicle')
    (individual_def individual 'TestVehicle2' :> 'Vehicle')
    (individual_usage individual 'testSystem' : 'TestSystem' :> 'massVerificationSystem'
      (portion_usage timeslice 'test1'
        (perform_action :>> 'vehicleMassTest'
          (individual_usage in individual :>> 'testVehicle' : 'TestVehicle1'
            (default_ref_usage :>> 'mass' value))))
      (source_succession
        (portion_usage timeslice 'test2'
          (perform_action :>> 'vehicleMassTest'
            (individual_usage in individual :>> 'testVehicle' : 'TestVehicle2'
              (default_ref_usage :>> 'mass' value))))))))
~~~
# FORMAT
~~~sysml
package 'Verification Case Usage Example' {
    private import 'Verification Case Definition Example'::*;

    part def MassVerificationSystem;
    part def Scale;

    part vehicleTestConfig : Vehicle {
        // ...
    }

    verification vehicleMassTest : VehicleMassTest {
        subject testVehicle :> vehicleTestConfig;
    }

    part massVerificationSystem : MassVerificationSystem {
        perform :>> vehicleMassTest;

        part scale : Scale {
            perform :>> vehicleMassTest.collectData {
                in part :>> testVehicle;

                // In reality, this would be some more involved process.
                measurement = testVehicle.mass;

                out :>> massMeasured = measurement;
            }
        }
    }

    individual def TestSystem :> MassVerificationSystem;

    individual def TestVehicle1 :> Vehicle;
    individual def TestVehicle2 :> Vehicle;

    individual testSystem : TestSystem :> massVerificationSystem {
        timeslice test1 {
            perform action :>> vehicleMassTest {
                in individual :>> testVehicle : TestVehicle1 {
                    :>> mass = 2500[SI::kg];
                }
            }
        }

        then timeslice test2 {
			perform action :>> vehicleMassTest {
				in individual :>> testVehicle : TestVehicle2 {
					:>> mass = 3000[SI::kg];
				}
			}
		}
    }
}
~~~
# EXPECTED
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'VehicleMassTest'
semantic.unresolved_name 'vehicleMassTest::collectData'
semantic.unresolved_name 'testVehicle'
semantic.unresolved_name 'massMeasured'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'VehicleMassTest'
semantic.unresolved_name 'vehicleMassTest::collectData'
semantic.unresolved_name 'testVehicle'
semantic.unresolved_name 'massMeasured'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'Vehicle'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'mass'
~~~
# SMG
~~~
(model
  (namespace
    (package 'Verification Case Usage Example'
      (namespace_import private -> 'Verification Case Definition Example'[unresolved])
      (part_def 'MassVerificationSystem')
      (part_def 'Scale')
      (part_usage 'vehicleTestConfig' : 'Vehicle'[unresolved])
      (verification_case_usage 'vehicleMassTest' : 'VehicleMassTest'[unresolved]
        (subject_membership in 'testVehicle' :> 'Verification Case Usage Example::vehicleTestConfig'[part_usage]))
      (part_usage 'massVerificationSystem' : 'Verification Case Usage Example::MassVerificationSystem'[part_def]
        (perform_action_usage :>> 'Verification Case Usage Example::vehicleMassTest'[verification_case_usage])
        (part_usage composite 'scale' : 'Verification Case Usage Example::Scale'[part_def]
          (perform_action_usage :>> 'vehicleMassTest::collectData'[unresolved]
            (part_usage in :>> 'testVehicle'[unresolved])
            (reference_usage reference 'measurement'
              (feature_value (=)))
            (reference_usage out reference :>> 'massMeasured'[unresolved]
              (feature_value (=))))))
      (occurrence_def individual 'TestSystem' :> 'Verification Case Usage Example::MassVerificationSystem'[part_def])
      (occurrence_def individual 'TestVehicle1' :> 'Vehicle'[unresolved])
      (occurrence_def individual 'TestVehicle2' :> 'Vehicle'[unresolved])
      (occurrence_usage individual 'testSystem' : 'Verification Case Usage Example::TestSystem'[occurrence_def] :> 'Verification Case Usage Example::massVerificationSystem'[part_usage]
        (occurrence_usage composite 'test1'
          (perform_action_usage :>> ''[perform_action_usage]
            (occurrence_usage in individual :>> 'Verification Case Usage Example::vehicleMassTest::testVehicle'[subject_membership] : 'Verification Case Usage Example::TestVehicle1'[occurrence_def]
              (reference_usage reference :>> 'mass'[unresolved]
                (feature_value (=))))))
        (source_succession
          (occurrence_usage 'test2'
            (perform_action_usage :>> ''[perform_action_usage]
              (occurrence_usage in individual :>> 'Verification Case Usage Example::vehicleMassTest::testVehicle'[subject_membership] : 'Verification Case Usage Example::TestVehicle2'[occurrence_def]
                (reference_usage reference :>> 'mass'[unresolved]
                  (feature_value (=)))))))))))
~~~
