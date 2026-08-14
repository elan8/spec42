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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 6) (end 12 16))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 6) (end 13 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_verification_case_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 38))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 32 34) (end 32 58))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 37 35) (end 37 60))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 17) (end 38 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 40 4) (end 40 10))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 43 19) (end 43 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 43 33) (end 43 53))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness parse-recovery) (has-evaluation true) (source-digest "blake3:fb343b407666e6c8be3f68dca4023d3d7d4c39c5cff537d3038d0b30f56d0c2b") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "ISQ::mass")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (kind verification-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction out)))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind part) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction in)) (memberAccessOperand (reference "processData::massProcessed")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind") (direction out)) (expressionOperand (reference "testVehicle")) (expressionOperand (reference "massProcessed")) (invocationCallee (reference "PassIf")) (invocationCallee (reference "vehicleMassRequirement")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData"))) (kind action) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction in)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction in)) (memberAccessOperand (reference "collectData::massMeasured")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind parameter) (membership (kind feature) (visibility default)) (facts (direction out)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass") (direction out)))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind verification-def) (name "VehicleMassTest")) (named (kind requirement) (name "vehicleMassVerificationObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verify-requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (verifyRequirementTarget (reference "vehicleMassRequirement")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::verdict"))) (kind parameter) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "VerdictKind")) (memberAccessOperand (reference "evaluateData::verdict")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " The vehicle mass shall be less than or equal to 2500 kg. "))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind require-constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "massActual")) (expressionOperand (reference "massActual")) (memberAccessOperand (reference "vehicle::mass")))))
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
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
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "processData::massProcessed")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerdictKind")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind expressionOperand) (ordinal 0))
      (authored-target "testVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind expressionOperand) (ordinal 1))
      (authored-target "massProcessed")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind invocationCallee) (ordinal 0))
      (authored-target "PassIf")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind invocationCallee) (ordinal 1))
      (authored-target "vehicleMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "collectData::massMeasured")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind verification-def) (name "VehicleMassTest")) (named (kind requirement) (name "vehicleMassVerificationObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0))
      (authored-target "vehicleMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::verdict"))) (kind featureTyping) (ordinal 0))
      (authored-target "VerdictKind")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::verdict"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "evaluateData::verdict")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0))
      (authored-target "massActual")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1))
      (authored-target "massActual")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind expressionOperand) (ordinal 1)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind invocationCallee) (ordinal 1)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind verifyRequirementTarget) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind verification-def) (name "VehicleMassTest")) (named (kind requirement) (name "vehicleMassVerificationObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind verification-def) (name "VehicleMassTest")) (named (kind requirement) (name "vehicleMassVerificationObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (state non-constant))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))
      (subtype (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData")))
      (type (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest")))
      (type (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind verification-def) (name "VehicleMassTest")) (named (kind requirement) (name "vehicleMassVerificationObjective")) (anonymous (kind verify-requirement) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::verdict")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")))
    )
    (declaration (id (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle")))
      (featured-by (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")))
      (type (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")) (scopes any))
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
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 28 23) (end 28 32)) (probe (position 28 23))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 27 25) (end 27 32)) (probe (position 27 25))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 37 23) (end 37 32)) (probe (position 37 23))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 37 35) (end 37 60)) (probe (position 37 35))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind memberAccessOperand) (ordinal 0) (authored-target "processData::massProcessed")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 38 17) (end 38 28)) (probe (position 38 17))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0) (authored-target "VerdictKind")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 40 44) (end 40 55)) (probe (position 40 44))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind expressionOperand) (ordinal 0) (authored-target "testVehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 40 70) (end 40 83)) (probe (position 40 70))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind expressionOperand) (ordinal 1) (authored-target "massProcessed")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 40 4) (end 40 10)) (probe (position 40 4))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind invocationCallee) (ordinal 0) (authored-target "PassIf")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 40 11) (end 40 33)) (probe (position 40 11))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind invocationCallee) (ordinal 1) (authored-target "vehicleMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 32 22) (end 32 31)) (probe (position 32 22))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 32 34) (end 32 58)) (probe (position 32 34))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind memberAccessOperand) (ordinal 0) (authored-target "collectData::massMeasured")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 33 24) (end 33 33)) (probe (position 33 24))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 20 24) (end 20 31)) (probe (position 20 24))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 23 10) (end 23 32)) (probe (position 23 10))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind verification-def) (name "VehicleMassTest")) (named (kind requirement) (name "vehicleMassVerificationObjective")) (anonymous (kind verify-requirement) (ordinal 0))))) (kind verifyRequirementTarget) (ordinal 0) (authored-target "vehicleMassRequirement")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 43 19) (end 43 30)) (probe (position 43 19))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::verdict"))) (kind featureTyping) (ordinal 0) (authored-target "VerdictKind")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 43 33) (end 43 53)) (probe (position 43 33))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::VehicleMassTest::verdict"))) (kind memberAccessOperand) (ordinal 0) (authored-target "evaluateData::verdict")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 12 6) (end 12 16)) (probe (position 12 6))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 0) (authored-target "massActual")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 13 6) (end 13 16)) (probe (position 13 6))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind expressionOperand) (ordinal 1) (authored-target "massActual")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 12 20) (end 12 32)) (probe (position 12 20))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (path (named (kind package) (name "Verification Case Definition Example")) (named (kind requirement) (name "vehicleMassRequirement")) (anonymous (kind require-constraint) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle::mass")))))
    )
  )
  (query (document "memory://snapshot/34_verification_case_definition_example.md") (range (start 7 20) (end 7 27)) (probe (position 7 20))
    (reference (id (source (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/34_verification_case_definition_example.md") (qualified-name "Verification Case Definition Example::Vehicle")))))
    )
  )
)
~~~
