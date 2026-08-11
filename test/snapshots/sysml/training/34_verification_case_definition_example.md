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
  (document "34_verification_case_definition_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 3 20) (end 3 29))
      )
      (diagnostic
        (severity warning)
        (code "analysis_evaluation_unresolved")
        (source "semantic")
        (range (start 6 1) (end 6 270))
      )
      (diagnostic
        (severity error)
        (code "unexpected_keyword_in_scope")
        (source "sysml")
        (range (start 8 2) (end 8 32))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 27 3) (end 27 64))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 28 3) (end 28 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 32 3) (end 32 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 33 3) (end 33 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 37 3) (end 37 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 38 3) (end 38 222))
      )
    )
  )
)
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "0ea66fbed9bb68b4c044c9b36f92357b55246ce854af476960e769d1b529ecd3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example"))) (kind "package") (name "Verification Case Definition Example") (declared-name "Verification Case Definition Example"))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "Verification Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (kind "verification def") (name "VehicleMassTest") (declared-name "VehicleMassTest") (parent (node (document "d0") (qualified-name "Verification Case Definition Example"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData"))) (kind "action") (name "collectData") (declared-name "collectData") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind "in out parameter") (name "massMeasured") (declared-name "massMeasured") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind "part") (name "testVehicle") (declared-name "testVehicle") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData"))) (kind "action") (name "evaluateData") (declared-name "evaluateData") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind "in out parameter") (name "massProcessed") (declared-name "massProcessed") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind "in out parameter") (name "verdict") (declared-name "verdict") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData"))) (authored (relationships (typing (reference "VerdictKind")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData"))) (kind "action") (name "processData") (declared-name "processData") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind "in out parameter") (name "massMeasured") (declared-name "massMeasured") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind "in out parameter") (name "massProcessed") (declared-name "massProcessed") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind "subject") (name "testVehicle") (declared-name "testVehicle") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective"))) (kind "objective") (name "vehicleMassVerificationObjective") (declared-name "vehicleMassVerificationObjective") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind "verified requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective"))) (authored (relationships (typing (reference "vehicleMassRequirement")) (subject (reference "vehicleMassRequirement")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (kind "requirement") (name "vehicleMassRequirement") (declared-name "vehicleMassRequirement") (parent (node (document "d0") (qualified-name "Verification Case Definition Example"))) (authored (membership (kind Feature)) (relationships (subject (reference "Verification Case Definition Example::vehicleMassRequirement::vehicle")))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::_requireConstraint_0"))) (kind "require constraint") (name "_requireConstraint_0") (declared-name "_requireConstraint_0") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))))
    (element (id (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (authored (relationships (typing (reference "Vehicle")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle::mass"))) (kind subsetting) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::massMeasured"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict"))) (kind featureTyping) (ordinal 0)) (authored-target "VerdictKind") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massProcessed"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)) (authored-target "vehicleMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "vehicleMassRequirement") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "Verification Case Definition Example::vehicleMassRequirement::vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::testVehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::vehicleMassVerificationObjective::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (target (node (document "d0") (qualified-name "Verification Case Definition Example::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::vehicle"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::massProcessed")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::evaluateData::verdict")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::processData::massMeasured")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement")) (expression (status "unresolved") (error "expression has an unresolved reference")) (analysis (status "unresolved")))
    (node (node (document "d0") (qualified-name "Verification Case Definition Example::vehicleMassRequirement::_requireConstraint_0")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 27 25) (end 27 32)) (probe (position 27 25))
      (reference
        (source (document "d0") (qualified-name "Verification Case Definition Example::VehicleMassTest::collectData::testVehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 27 25) (end 27 32))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Verification Case Definition Example::Vehicle") (range (start 2 1) (end 2 53)))
        )
      )
    )
    (query (range (start 3 20) (end 3 29)) (probe (position 3 20))
      (reference
        (source (document "d0") (qualified-name "Verification Case Definition Example::Vehicle::mass"))
        (kind subsetting) (ordinal 0) (authored-target "ISQ::mass")
        (range (start 3 20) (end 3 29))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
