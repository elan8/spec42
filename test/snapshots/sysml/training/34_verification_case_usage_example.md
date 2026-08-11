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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "3eac23f3b76759fa2fffdf9518204f6b1b7b7ed7888944d63ed785ace80f860e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example"))) (kind "package") (name "Verification Case Usage Example") (declared-name "Verification Case Usage Example") (range (start (line 0) (character 0)) (end (line 0) (character 1177))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 58))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Verification Case Definition Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 54))))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (kind "part def") (name "MassVerificationSystem") (declared-name "MassVerificationSystem") (range (start (line 3) (character 1)) (end (line 3) (character 33))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::Scale"))) (kind "part def") (name "Scale") (declared-name "Scale") (range (start (line 4) (character 1)) (end (line 4) (character 16))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind "individual def") (name "TestSystem") (declared-name "TestSystem") (range (start (line 29) (character 1)) (end (line 29) (character 53))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "MassVerificationSystem") (range (start (line 29) (character 30)) (end (line 29) (character 52)))))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind "individual def") (name "TestVehicle1") (declared-name "TestVehicle1") (range (start (line 31) (character 1)) (end (line 31) (character 40))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 31) (character 32)) (end (line 31) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind "individual def") (name "TestVehicle2") (declared-name "TestVehicle2") (range (start (line 32) (character 1)) (end (line 32) (character 40))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Owning)) (relationships (specializes (reference "Vehicle") (range (start (line 32) (character 32)) (end (line 32) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind "part") (name "massVerificationSystem") (declared-name "massVerificationSystem") (range (start (line 14) (character 1)) (end (line 14) (character 337))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassVerificationSystem") (range (start (line 14) (character 31)) (end (line 14) (character 53)))) (perform (reference "Verification Case Usage Example::massVerificationSystem::vehicleMassTest") (range none)))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind "part") (name "scale") (declared-name "scale") (range (start (line 17) (character 2)) (end (line 17) (character 248))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (authored (membership (kind Feature)) (relationships (typing (reference "Scale") (range (start (line 17) (character 15)) (end (line 17) (character 20)))) (perform (reference "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest::collectData") (range none)))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest.collectData"))) (kind "action") (name "vehicleMassTest.collectData") (declared-name "vehicleMassTest.collectData") (range (start (line 18) (character 3)) (end (line 18) (character 221))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest"))) (kind "action") (name "vehicleMassTest") (declared-name "vehicleMassTest") (range (start (line 15) (character 2)) (end (line 15) (character 26))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind "occurrence") (name "testSystem") (declared-name "testSystem") (range (start (line 34) (character 12)) (end (line 34) (character 386))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TestSystem") (range none)) (subsetting (reference "massVerificationSystem") (range (start (line 34) (character 39)) (end (line 34) (character 61)))))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem::test1"))) (kind "occurrence") (name "test1") (declared-name "test1") (range (start (line 35) (character 12)) (end (line 35) (character 155))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem::test2"))) (kind "occurrence") (name "test2") (declared-name "test2") (range (start (line 43) (character 17)) (end (line 43) (character 160))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind "verification") (name "vehicleMassTest") (declared-name "vehicleMassTest") (range (start (line 10) (character 1)) (end (line 10) (character 96))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "VehicleMassTest") (range none)))))
    (element (id (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind "part") (name "vehicleTestConfig") (declared-name "vehicleTestConfig") (range (start (line 6) (character 1)) (end (line 6) (character 47))) (parent (node (document "d0") (qualified-name "Verification Case Usage Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 6) (character 26)) (end (line 6) (character 33)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Verification Case Definition Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 54))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind specialization) (ordinal 0)) (authored-target "MassVerificationSystem") (range (start (line 29) (character 30)) (end (line 29) (character 52))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 31) (character 32)) (end (line 31) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind specialization) (ordinal 0)) (authored-target "Vehicle") (range (start (line 32) (character 32)) (end (line 32) (character 39))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "MassVerificationSystem") (range (start (line 14) (character 31)) (end (line 14) (character 53))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind performSource) (ordinal 0)) (authored-target "Verification Case Usage Example::massVerificationSystem::vehicleMassTest") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::vehicleMassTest")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)) (authored-target "Scale") (range (start (line 17) (character 15)) (end (line 17) (character 20))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::Scale")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind performSource) (ordinal 0)) (authored-target "Verification Case Usage Example::massVerificationSystem::scale::vehicleMassTest::collectData") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0)) (authored-target "TestSystem") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::TestSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0)) (authored-target "massVerificationSystem") (range (start (line 34) (character 39)) (end (line 34) (character 61))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Usage Example::massVerificationSystem")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind featureTyping) (ordinal 0)) (authored-target "VehicleMassTest") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 6) (character 26)) (end (line 6) (character 33))) (outcome (status unresolved)))
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
