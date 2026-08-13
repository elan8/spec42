# META
~~~ini
description=SysML Training 34 (Verification): Verification Case Definition Example
type=file
~~~
# SOURCE
~~~sysml
package 'Verification Case Definition Example' {
	
	part def Vehicle {
		attribute mass :> ISQ::mass;
	}
	
	requirement vehicleMassRequirement {
		subject vehicle : Vehicle;
		in massActual :> ISQ::mass;
		doc /* The vehicle mass shall be less than or equal to 2500 kg. */
		
		require constraint { 
		    massActual == vehicle.mass and 
		    massActual <= 2500[SI::kg]
		}
	}
		
	verification def VehicleMassTest {
		private import VerificationCases::*;

		subject testVehicle : Vehicle;
		objective vehicleMassVerificationObjective {
			// The subject of the verify is automatically bound to 'testVehicle' here.
			verify vehicleMassRequirement;
		}
		
		action collectData {
			in part testVehicle : Vehicle = VehicleMassTest::testVehicle;
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
				PassIf(vehicleMassRequirement(vehicle = testVehicle, massActual = massProcessed));
		}
		
		return verdict : VerdictKind = evaluateData.verdict;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/34_verification_case_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 20) (end 3 29))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "parser")
        (range (start 8 2) (end 9 2))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 11 2) (end 14 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 38))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 21 2) (end 24 3))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 23) (end 28 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 22) (end 32 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 24) (end 33 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 23) (end 37 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 17) (end 38 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 43 2) (end 43 54))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation false) (source-digest "blake3:fb343b407666e6c8be3f68dca4023d3d7d4c39c5cff537d3038d0b30f56d0c2b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction out))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction in))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind") (direction out))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction in))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction out))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (kind subsetting) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerdictKind")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 3 20) (end 3 29)) (probe (position 3 20))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 28 23) (end 28 32)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 27 25) (end 27 32)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 37 23) (end 37 32)) (probe (position 37 23))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 38 17) (end 38 28)) (probe (position 38 17))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0) (authored-target "VerdictKind")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 32 22) (end 32 31)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 33 24) (end 33 33)) (probe (position 33 24))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 20 24) (end 20 31)) (probe (position 20 24))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 7 20) (end 7 27)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
  )
)
~~~
