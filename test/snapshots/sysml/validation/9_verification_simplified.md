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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "3ab05ac8c5711a50a444f7f5e7f3c00424af88e397cb8d412044e50a70b14248") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified"))) (kind "package") (name "9-Verification-simplified") (declared-name "9-Verification-simplified"))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "9-Verification-simplified"))) (authored (membership (kind Import) (visibility "private") (import (reference "VerificationCases::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "9-Verification-simplified"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "9-Verification-simplified"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (kind "requirement def") (name "MassRequirement") (declared-name "MassRequirement") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (authored (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (authored (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (kind "verification def") (name "MassTest") (declared-name "MassTest") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective"))) (kind "objective") (name "massVerificationObjective") (declared-name "massVerificationObjective") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind "verified requirement") (name "MassRequirement") (declared-name "MassRequirement") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective"))) (authored (relationships (typing (reference "MassRequirement")) (subject (reference "MassRequirement")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (kind "part def") (name "MassVerificationSystem") (declared-name "MassVerificationSystem") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale"))) (kind "part def") (name "Scale") (declared-name "Scale") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))) (kind "part def") (name "TestOperator") (declared-name "TestOperator") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (kind "individual def") (name "TestSystem") (declared-name "TestSystem") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassVerificationSystem")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (kind "individual def") (name "TestVehicle1") (declared-name "TestVehicle1") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (kind "individual def") (name "TestVehicle2") (declared-name "TestVehicle2") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "9-Verification-simplified"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind "part") (name "massVerificationSystem") (declared-name "massVerificationSystem") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassVerificationSystem")) (perform (reference "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind "part") (name "scale") (declared-name "scale") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scale")) (perform (reference "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest::collectData")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest.collectData"))) (kind "action") (name "vehicleMassTest.collectData") (declared-name "vehicleMassTest.collectData") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind "part") (name "testOperator") (declared-name "testOperator") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestOperator")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest"))) (kind "action") (name "vehicleMassTest") (declared-name "vehicleMassTest") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind "ref") (name "vehicleUnderTest") (declared-name "vehicleUnderTest") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind "occurrence") (name "testSystem") (declared-name "testSystem") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestSystem")) (subsetting (reference "massVerificationSystem")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1"))) (kind "occurrence") (name "test1") (declared-name "test1") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestVehicle1")) (subsetting (reference "vehicle1_c2")) (redefinition (reference "vehicleUnderTest")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2"))) (kind "occurrence") (name "test2") (declared-name "test2") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind "occurrence") (name "") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestVehicle2")) (subsetting (reference "vehicle1_c2")) (redefinition (reference "vehicleUnderTest")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind "part") (name "vehicle1_c2") (declared-name "vehicle1_c2") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRequirement")) (subject (reference "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (kind "attribute") (name "massActual") (declared-name "massActual") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (authored (relationships (redefinition (reference "massActual")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (kind "attribute") (name "massReqd") (declared-name "massReqd") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (authored (relationships (redefinition (reference "massReqd")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (kind "verification") (name "vehicleMassTest") (declared-name "vehicleMassTest") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassTest")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (kind "action") (name "collectData") (declared-name "collectData") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::massMeasured"))) (kind "in out parameter") (name "massMeasured") (declared-name "massMeasured") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (kind "part") (name "testVehicle") (declared-name "testVehicle") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (kind "action") (name "evaluateData") (declared-name "evaluateData") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::massProcessed"))) (kind "in out parameter") (name "massProcessed") (declared-name "massProcessed") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::verdict"))) (kind "in out parameter") (name "verdict") (declared-name "verdict") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData"))) (authored (relationships (typing (reference "VerdictKind")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (kind "action") (name "processData") (declared-name "processData") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massMeasured"))) (kind "in out parameter") (name "massMeasured") (declared-name "massMeasured") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massProcessed"))) (kind "in out parameter") (name "massProcessed") (declared-name "massProcessed") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (kind "subject") (name "testVehicle") (declared-name "testVehicle") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective"))) (kind "objective") (name "vehicleMassVerificationObjective") (declared-name "vehicleMassVerificationObjective") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind "verified requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (parent (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective"))) (authored (relationships (typing (reference "vehicleMassRequirement")) (subject (reference "vehicleMassRequirement")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "VerificationCases::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "MassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest::massVerificationObjective::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))) (kind specialization) (ordinal 0)) (authored-target "MassVerificationSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "MassVerificationSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind performSource) (ordinal 0)) (authored-target "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleMassTest")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)) (authored-target "Scale") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind performSource) (ordinal 0)) (authored-target "9-Verification-simplified::Usages::massVerificationSystem::scale::vehicleMassTest::collectData") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind featureTyping) (ordinal 0)) (authored-target "TestOperator") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "TestSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind subsetting) (ordinal 0)) (authored-target "massVerificationSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind featureTyping) (ordinal 0)) (authored-target "TestVehicle1") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c2") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))) (kind redefinition) (ordinal 0)) (authored-target "vehicleUnderTest") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind featureTyping) (ordinal 0)) (authored-target "TestVehicle2") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind subsetting) (ordinal 0)) (authored-target "vehicle1_c2") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))) (kind redefinition) (ordinal 0)) (authored-target "vehicleUnderTest") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))) (kind redefinition) (ordinal 0)) (authored-target "massActual") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))) (kind redefinition) (ordinal 0)) (authored-target "massReqd") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest"))) (kind featureTyping) (ordinal 0)) (authored-target "MassTest") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassTest")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::massMeasured"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::massProcessed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0)) (authored-target "VerdictKind") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "vehicleMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "vehicleMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement")))))
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
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 86 16) (end 86 21)) (probe (position 86 16))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))
        (kind featureTyping) (ordinal 0) (authored-target "Scale")
        (range (start 86 16) (end 86 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::Scale") (range (start 20 2) (end 20 17)))
        )
      )
    )
    (query (range (start 23 33) (end 23 40)) (probe (position 23 33))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle1"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 23 33) (end 23 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle") (range (start 15 2) (end 15 56)))
        )
      )
    )
    (query (range (start 24 33) (end 24 40)) (probe (position 24 33))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestVehicle2"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 24 33) (end 24 40))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle") (range (start 15 2) (end 15 56)))
        )
      )
    )
    (query (range (start 46 21) (end 46 28)) (probe (position 46 21))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 46 21) (end 46 28))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle") (range (start 15 2) (end 15 56)))
        )
      )
    )
    (query (range (start 58 26) (end 58 33)) (probe (position 58 26))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassTest::collectData::testVehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 58 26) (end 58 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle") (range (start 15 2) (end 15 56)))
        )
      )
    )
    (query (range (start 82 31) (end 82 38)) (probe (position 82 31))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 82 31) (end 82 38))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle") (range (start 15 2) (end 15 56)))
        )
      )
    )
    (query (range (start 101 5) (end 101 13)) (probe (position 101 5))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 101 5) (end 101 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::::mass") (range (start 101 5) (end 101 30)))
        )
      )
    )
    (query (range (start 107 5) (end 107 13)) (probe (position 107 5))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 107 5) (end 107 13))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::::mass") (range (start 107 5) (end 107 30)))
        )
      )
    )
    (query (range (start 7 27) (end 7 36)) (probe (position 7 27))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 7 27) (end 7 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 8 25) (end 8 34)) (probe (position 8 25))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 8 25) (end 8 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 16 21) (end 16 30)) (probe (position 16 21))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 16 21) (end 16 30))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 27)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 2 16) (end 2 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions") (range (start 4 1) (end 4 718)))
        )
      )
    )
    (query (range (start 100 58) (end 100 69)) (probe (position 100 58))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1_c2")
        (range (start 100 58) (end 100 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2") (range (start 46 2) (end 46 44)))
        )
      )
    )
    (query (range (start 106 58) (end 106 69)) (probe (position 106 58))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))
        (kind subsetting) (ordinal 0) (authored-target "vehicle1_c2")
        (range (start 106 58) (end 106 69))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2") (range (start 46 2) (end 46 44)))
        )
      )
    )
    (query (range (start 43 3) (end 43 15)) (probe (position 43 3))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd"))
        (kind redefinition) (ordinal 0) (authored-target "massReqd")
        (range (start 43 3) (end 43 15))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massReqd") (range (start 43 3) (end 43 32)))
        )
      )
    )
    (query (range (start 84 23) (end 84 35)) (probe (position 84 23))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))
        (kind featureTyping) (ordinal 0) (authored-target "TestOperator")
        (range (start 84 23) (end 84 35))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestOperator") (range (start 21 2) (end 21 24)))
        )
      )
    )
    (query (range (start 42 3) (end 42 17)) (probe (position 42 3))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual"))
        (kind redefinition) (ordinal 0) (authored-target "massActual")
        (range (start 42 3) (end 42 17))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement::massActual") (range (start 42 3) (end 42 33)))
        )
      )
    )
    (query (range (start 100 23) (end 100 39)) (probe (position 100 23))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test1::"))
        (kind redefinition) (ordinal 0) (authored-target "vehicleUnderTest")
        (range (start 100 23) (end 100 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 106 23) (end 106 39)) (probe (position 106 23))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem::test2::"))
        (kind redefinition) (ordinal 0) (authored-target "vehicleUnderTest")
        (range (start 106 23) (end 106 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 33)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "VerificationCases::*")
        (range (start 1 16) (end 1 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 26 31) (end 26 53)) (probe (position 26 31))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Definitions::TestSystem"))
        (kind specialization) (ordinal 0) (authored-target "MassVerificationSystem")
        (range (start 26 31) (end 26 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem") (range (start 19 2) (end 19 34)))
        )
      )
    )
    (query (range (start 77 32) (end 77 54)) (probe (position 77 32))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))
        (kind featureTyping) (ordinal 0) (authored-target "MassVerificationSystem")
        (range (start 77 32) (end 77 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem") (range (start 19 2) (end 19 34)))
        )
      )
    )
    (query (range (start 98 40) (end 98 62)) (probe (position 98 40))
      (reference
        (source (document "d0") (qualified-name "9-Verification-simplified::Usages::testSystem"))
        (kind subsetting) (ordinal 0) (authored-target "massVerificationSystem")
        (range (start 98 40) (end 98 62))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem") (range (start 77 2) (end 77 490)))
        )
      )
    )
  )
)
~~~
