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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "34_verification_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 54))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 17 2) (end 17 248))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 31 32) (end 31 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_specializes_reference")
        (source "semantic")
        (range (start 32 32) (end 32 39))
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "3eac23f3b76759fa2fffdf9518204f6b1b7b7ed7888944d63ed785ace80f860e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example"))) (kind "package") (name "Verification Case Usage Example") (declared-name "Verification Case Usage Example"))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Verification Case Definition Example::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (kind "part def") (name "MassVerificationSystem") (declared-name "MassVerificationSystem") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::Scale"))) (kind "part def") (name "Scale") (declared-name "Scale") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind "individual def") (name "TestSystem") (declared-name "TestSystem") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassVerificationSystem")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind "individual def") (name "TestVehicle1") (declared-name "TestVehicle1") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind "individual def") (name "TestVehicle2") (declared-name "TestVehicle2") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind "part") (name "massVerificationSystem") (declared-name "massVerificationSystem") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassVerificationSystem")) (perform (reference "Verification Case Usage Example::massVerificationSystem::vehicleMassTest")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind "part") (name "scale") (declared-name "scale") (parent (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scale")) (perform (reference "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest::collectData")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest.collectData"))) (kind "action") (name "vehicleMassTest.collectData") (declared-name "vehicleMassTest.collectData") (parent (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest"))) (kind "action") (name "vehicleMassTest") (declared-name "vehicleMassTest") (parent (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind "occurrence") (name "testSystem") (declared-name "testSystem") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestSystem")) (subsetting (reference "massVerificationSystem")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem::test1"))) (kind "occurrence") (name "test1") (declared-name "test1") (parent (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem::test2"))) (kind "occurrence") (name "test2") (declared-name "test2") (parent (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind "verification") (name "vehicleMassTest") (declared-name "vehicleMassTest") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleMassTest")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind "part") (name "vehicleTestConfig") (declared-name "vehicleTestConfig") (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Verification Case Definition Example::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind specialization) (ordinal 0)) (authored-target "MassVerificationSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "MassVerificationSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind performSource) (ordinal 0)) (authored-target "Verification Case Usage Example::massVerificationSystem::vehicleMassTest") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)) (authored-target "Scale") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::Scale")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind performSource) (ordinal 0)) (authored-target "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest::collectData") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "TestSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0)) (authored-target "massVerificationSystem") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleMassTest") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (target (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (target (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind perform) (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (target (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind performSource) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (target (node (document "d0") (qualified-name "Verification Case Usage Example::Scale"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (target (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (target (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 17 15) (end 17 20)) (probe (position 17 15))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))
        (kind featureTyping) (ordinal 0) (authored-target "Scale")
        (range (start 17 15) (end 17 20))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Verification Case Usage Example::Scale") (range (start 4 1) (end 4 16)))
        )
      )
    )
    (query (range (start 6 26) (end 6 33)) (probe (position 6 26))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 6 26) (end 6 33))
        (outcome (status unresolved))
      )
    )
    (query (range (start 31 32) (end 31 39)) (probe (position 31 32))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle1"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 31 32) (end 31 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 32 32) (end 32 39)) (probe (position 32 32))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle2"))
        (kind specialization) (ordinal 0) (authored-target "Vehicle")
        (range (start 32 32) (end 32 39))
        (outcome (status unresolved))
      )
    )
    (query (range (start 14 31) (end 14 53)) (probe (position 14 31))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))
        (kind featureTyping) (ordinal 0) (authored-target "MassVerificationSystem")
        (range (start 14 31) (end 14 53))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem") (range (start 3 1) (end 3 33)))
        )
      )
    )
    (query (range (start 29 30) (end 29 52)) (probe (position 29 30))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))
        (kind specialization) (ordinal 0) (authored-target "MassVerificationSystem")
        (range (start 29 30) (end 29 52))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem") (range (start 3 1) (end 3 33)))
        )
      )
    )
    (query (range (start 34 39) (end 34 61)) (probe (position 34 39))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))
        (kind subsetting) (ordinal 0) (authored-target "massVerificationSystem")
        (range (start 34 39) (end 34 61))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem") (range (start 14 1) (end 14 337)))
        )
      )
    )
    (query (range (start 1 16) (end 1 54)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Verification Case Usage Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Verification Case Definition Example::*")
        (range (start 1 16) (end 1 54))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
