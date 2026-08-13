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
  (document "memory://snapshot/9_verification_simplified.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 7 27) (end 7 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 8 25) (end 8 34))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 12 3) (end 12 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 16 21) (end 16 30))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 23 2) (end 23 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 24 2) (end 24 41))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 26 2) (end 26 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 29 3) (end 31 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 39 3) (end 39 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 42 7) (end 42 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 43 7) (end 43 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_package_member")
        (source "semantic")
        (range (start 50 2) (end 75 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 78 3) (end 80 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 87 4) (end 94 5))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 98 26) (end 98 36))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 98 40) (end 98 62))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 100 23) (end 100 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 100 42) (end 100 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 100 58) (end 100 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 101 9) (end 101 13))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 106 23) (end 106 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 106 42) (end 106 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 106 58) (end 106 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_reference")
        (source "semantic")
        (range (start 107 9) (end 107 13))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:a71b1b70856686bf9eb79aa546db832913d67e9f20d25a851fe09958c0d483c9") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "VerificationCases") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassTest"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Scale"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassVerificationSystem"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Scale"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TestOperator"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TestSystem")) (subsetting (reference "massVerificationSystem"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem::test1"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TestVehicle1")) (subsetting (reference "vehicle1_c2")) (redefinition (reference "vehicleUnderTest"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem::test2"))) (kind occurrence) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind occurrence) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TestVehicle2")) (subsetting (reference "vehicle1_c2")) (redefinition (reference "vehicleUnderTest"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRequirement"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massActual"))))
    (declaration (id (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "massReqd"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "VerificationCases")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0))
      (authored-target "Scale")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Scale")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind featureTyping) (ordinal 0))
      (authored-target "TestOperator")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::TestOperator")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind featureTyping) (ordinal 0))
      (authored-target "TestSystem")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind subsetting) (ordinal 0))
      (authored-target "massVerificationSystem")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TestVehicle1")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_c2")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "vehicleUnderTest")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "TestVehicle2")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind subsetting) (ordinal 0))
      (authored-target "vehicle1_c2")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "vehicleUnderTest")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "massActual")
      (outcome (status unsupported)))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "massReqd")
      (outcome (status unsupported)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Scale"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::TestOperator"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 1 16) (end 1 36)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "VerificationCases")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 2 16) (end 2 30)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 7 27) (end 7 36)) (probe (position 7 27))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massActual"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 8 25) (end 8 34)) (probe (position 8 25))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement::massReqd"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 16 21) (end 16 30)) (probe (position 16 21))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 77 32) (end 77 54)) (probe (position 77 32))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem"))) (kind featureTyping) (ordinal 0) (authored-target "MassVerificationSystem")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassVerificationSystem")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 86 16) (end 86 21)) (probe (position 86 16))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::scale"))) (kind featureTyping) (ordinal 0) (authored-target "Scale")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Scale")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 84 23) (end 84 35)) (probe (position 84 23))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::testOperator"))) (kind featureTyping) (ordinal 0) (authored-target "TestOperator")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::TestOperator")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 82 31) (end 82 38)) (probe (position 82 31))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::massVerificationSystem::vehicleUnderTest"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 98 26) (end 98 36)) (probe (position 98 26))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind featureTyping) (ordinal 0) (authored-target "TestSystem")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 98 40) (end 98 62)) (probe (position 98 40))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::testSystem"))) (kind subsetting) (ordinal 0) (authored-target "massVerificationSystem")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 100 42) (end 100 54)) (probe (position 100 42))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TestVehicle1")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 100 58) (end 100 69)) (probe (position 100 58))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_c2")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 100 23) (end 100 39)) (probe (position 100 23))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "vehicleUnderTest")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 101 9) (end 101 13)) (probe (position 101 9))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 106 42) (end 106 54)) (probe (position 106 42))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "TestVehicle2")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 106 58) (end 106 69)) (probe (position 106 58))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind subsetting) (ordinal 0) (authored-target "vehicle1_c2")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 106 23) (end 106 39)) (probe (position 106 23))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind occurrence) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "vehicleUnderTest")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 107 9) (end 107 13)) (probe (position 107 9))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 46 21) (end 46 28)) (probe (position 46 21))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicle1_c2"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 38 47) (end 38 62)) (probe (position 38 47))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Usages::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0) (authored-target "MassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/9_verification_simplified.md") (qualified-name "9-Verification-simplified::Definitions::MassRequirement")))))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 42 7) (end 42 17)) (probe (position 42 7))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "massActual")
      (outcome (status unsupported)))
  )
  (query (document "memory://snapshot/9_verification_simplified.md") (range (start 43 7) (end 43 15)) (probe (position 43 7))
    (reference (id (source (node (document "memory://snapshot/9_verification_simplified.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "massReqd")
      (outcome (status unsupported)))
  )
)
~~~
