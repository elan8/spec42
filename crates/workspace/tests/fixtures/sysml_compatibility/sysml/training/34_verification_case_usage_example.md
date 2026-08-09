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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Verification Case Usage Example"))) (name "Verification Case Usage Example") (declared-name "Verification Case Usage Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Verification Case Usage Example::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (name "MassVerificationSystem") (declared-name "MassVerificationSystem") (declared))
        (element (kind "part def") (id (node (document "d0") (qualified-name "Verification Case Usage Example::Scale"))) (name "Scale") (declared-name "Scale") (declared))
        (element (kind "individual def") (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (name "TestSystem") (declared-name "TestSystem"))
        (element (kind "individual def") (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (name "TestVehicle1") (declared-name "TestVehicle1"))
        (element (kind "individual def") (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (name "TestVehicle2") (declared-name "TestVehicle2"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (name "massVerificationSystem") (declared-name "massVerificationSystem") (declared (properties (composite true) (reference false) (ordered false)))
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (name "scale") (declared-name "scale") (declared (properties (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (featuring-type (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))))
              (contains
                (element (kind "action") (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest.collectData"))) (name "vehicleMassTest.collectData") (declared-name "vehicleMassTest.collectData") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Usage Example::Scale")))))
              )
            )
            (element (kind "action") (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest"))) (name "vehicleMassTest") (declared-name "vehicleMassTest") (effective (featuring-type (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
          )
        )
        (element (kind "occurrence") (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (name "testSystem") (declared-name "testSystem") (declared (properties (individual true) (composite true) (reference false)))
          (contains
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem::test1"))) (name "test1") (declared-name "test1") (declared (properties (portion true) (composite true) (reference false) (portion-kind "timeslice"))))
            (element (kind "occurrence") (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem::test2"))) (name "test2") (declared-name "test2") (declared (properties (portion true) (composite true) (reference false) (portion-kind "timeslice"))))
          )
        )
        (element (kind "verification") (id (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (name "vehicleMassTest") (declared-name "vehicleMassTest"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (name "vehicleTestConfig") (declared-name "vehicleTestConfig") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (perform (status resolved) (from (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (to (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest"))))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (to (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (to (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (to (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (to (node (document "d0") (qualified-name "Verification Case Usage Example::Scale"))))
  )
  (pending-relationships
    (perform (status pending) (document "d0") (source-qualified "Verification Case Usage Example::massVerificationSystem::scale") (target-qualified "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest::collectData"))
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/training/34_verification_case_usage_example.md"
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
        (range (start 1 1) (end 1 58))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 26) (end 6 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 1) (end 10 96))
      )
      (diagnostic
        (severity warning)
        (code "incompatible_specializes_kind")
        (source "semantic")
        (range (start 29 1) (end 29 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 1) (end 31 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 1) (end 32 40))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 36 3) (end 36 134))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 44 3) (end 44 134))
      )
    )
  )
)
~~~
