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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 32) (end 10 47))
      )
      (diagnostic
        (severity error)
        (code "recovered_use_case_body_element")
        (source "parser")
        (range (start 11 2) (end 11 43))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 16) (end 19 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 22 18) (end 22 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 24 12) (end 24 24))
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
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:dac6ccdc5393d51bccd3426b33db91459a15062489af182d8f18465c53f7d8af") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Verification Case Definition Example") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind individual-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "MassVerificationSystem")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind individual-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind individual-definition) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassVerificationSystem")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scale")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0))))) (kind perform-action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (facts (direction in)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "testVehicle")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massMeasured")) (expressionOperand (reference "measurement")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement"))))) (kind default-reference) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (memberAccessOperand (reference "testVehicle::mass")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (modifiers individual)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TestSystem")) (subsetting (reference "massVerificationSystem")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem::test1"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem::test2"))) (kind occurrence) (membership (kind feature) (visibility default)) (facts (portion timeslice)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind verification) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VehicleMassTest")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Verification Case Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind specialization) (ordinal 0))
      (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind specialization) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scale")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "testVehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massMeasured")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "measurement")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement")))))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement"))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "testVehicle::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "TestSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0))
      (authored-target "massVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind featureTyping) (ordinal 0))
      (authored-target "VehicleMassTest")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind specialization) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement"))))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (state non-constant))
    (evaluated (declaration (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement"))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))
      (subtype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")))
      (subtype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")))
      (supertype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")) (scopes any subclassification))
      (subtype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))
      (type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")) (source direct))
      (supertype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")) (scopes any))
      (subtype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale")))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))
      (type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")) (provenance authored))
      (effective-type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")) (source direct))
      (supertype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind part) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement")))))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem")))
      (type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")) (provenance authored))
      (effective-type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")) (source inherited) (from (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))))
      (effective-type (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")) (source direct))
      (supertype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")) (scopes any))
      (supertype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")) (scopes any))
      (supertype (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem::test1")))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem::test2")))
      (featured-by (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 1 16) (end 1 57)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Verification Case Definition Example")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 29 30) (end 29 52)) (probe (position 29 30))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem"))) (kind specialization) (ordinal 0) (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 31 32) (end 31 39)) (probe (position 31 32))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestVehicle1"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 32 32) (end 32 39)) (probe (position 32 32))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestVehicle2"))) (kind specialization) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 14 31) (end 14 53)) (probe (position 14 31))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem"))) (kind featureTyping) (ordinal 0) (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::MassVerificationSystem")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 17 15) (end 17 20)) (probe (position 17 15))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0) (authored-target "Scale")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::Scale")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 19 16) (end 19 27)) (probe (position 19 16))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind part) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "testVehicle")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 24 12) (end 24 24)) (probe (position 24 12))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massMeasured")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 24 27) (end 24 38)) (probe (position 24 27))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (anonymous (kind parameter) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "measurement")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement")))))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 22 18) (end 22 34)) (probe (position 22 18))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (path (named (kind package) (name "Verification Case Usage Example")) (named (kind part) (name "massVerificationSystem")) (named (kind part) (name "scale")) (anonymous (kind perform-action) (ordinal 0)) (named (kind default-reference) (name "measurement"))))) (kind memberAccessOperand) (ordinal 0) (authored-target "testVehicle::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 34 25) (end 34 35)) (probe (position 34 25))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind featureTyping) (ordinal 0) (authored-target "TestSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::TestSystem")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 34 39) (end 34 61)) (probe (position 34 39))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::testSystem"))) (kind subsetting) (ordinal 0) (authored-target "massVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::massVerificationSystem")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 10 32) (end 10 47)) (probe (position 10 32))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleMassTest"))) (kind featureTyping) (ordinal 0) (authored-target "VehicleMassTest")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_usage_example.md") (range (start 6 26) (end 6 33)) (probe (position 6 26))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_usage_example.md") (qualified-name "Verification Case Usage Example::vehicleTestConfig"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
    )
  )
)
~~~
