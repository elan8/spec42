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
  (document "memory://snapshot/34_verification_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 57))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 6 26) (end 6 33))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 10 1) (end 12 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 15 2) (end 15 26))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 18 3) (end 25 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 29 1) (end 29 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 31 1) (end 31 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 32 1) (end 32 40))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 25) (end 34 35))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 36 3) (end 41 2))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 44 3) (end 49 2))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:dac6ccdc5393d51bccd3426b33db91459a15062489af182d8f18465c53f7d8af") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Verification Case Definition Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassVerificationSystem"))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scale"))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TestSystem")) (subsetting (reference "massVerificationSystem"))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem::test1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem::test2"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Verification Case Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scale")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "TestSystem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0))
      (authored-target "massVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 1 16) (end 1 57)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Verification Case Definition Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 14 31) (end 14 53)) (probe (position 14 31))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0) (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 17 15) (end 17 20)) (probe (position 17 15))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0) (authored-target "Scale")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")))))
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 34 25) (end 34 35)) (probe (position 34 25))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0) (authored-target "TestSystem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 34 39) (end 34 61)) (probe (position 34 39))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0) (authored-target "massVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))))
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 6 26) (end 6 33)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
)
~~~
