# META
~~~ini
description=SysML Training 33 (Analysis): Analysis Case Usage Example
type=file
~~~
# SOURCE
~~~sysml
package 'Analysis Case Usage Example' {
	private import 'Analysis Case Definition Example'::*;
	
	part vehicleFuelEconomyAnalysisContext {
		requirement vehicleFuelEconomyRequirements {
			subject vehicle : Vehicle;
			// ...
		}
		
		attribute cityScenario : WayPoint[*] = ( //* ... */ );
		attribute highwayScenario : WayPoint[*] = ( //* ... */ );
		
		analysis cityAnalysis : FuelEconomyAnalysis {
			subject vehicle = vehicle_c1;
			in scenario = cityScenario;
		}
		
		analysis highwayAnalysis : FuelEconomyAnalysis {
			subject vehicle = vehicle_c1;
			in scenario = highwayScenario;
		}
		
		part vehicle_c1 : Vehicle {
			// ...
			
			attribute :>> fuelEconomy_city = cityAnalysis.fuelEconomyResult;
			attribute :>> fuelEconomy_highway = highwayAnalysis.fuelEconomyResult;
		}
		
		satisfy vehicleFuelEconomyRequirements by vehicle_c1;
	}

}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "33_analysis_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 50))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 9 2) (end 9 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 2) (end 9 56))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 27) (end 9 35))
      )
      (diagnostic
        (severity error)
        (code "implicit_redefinition_without_operator")
        (source "semantic")
        (range (start 10 2) (end 10 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 59))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 30) (end 10 38))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 20) (end 22 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 29 10) (end 29 40))
      )
    )
  )
)
~~~
# FORMAT
~~~sysml
package 'Analysis Case Usage Example' {
	private import 'Analysis Case Definition Example'::*;
	
	part vehicleFuelEconomyAnalysisContext {
		requirement vehicleFuelEconomyRequirements {
			subject vehicle : Vehicle;
			// ...
		}
		
		attribute cityScenario : WayPoint[*] = ( //* ... */ );
		attribute highwayScenario : WayPoint[*] = ( //* ... */ );
		
		analysis cityAnalysis : FuelEconomyAnalysis {
			subject vehicle = vehicle_c1;
			in scenario = cityScenario;
		}
		
		analysis highwayAnalysis : FuelEconomyAnalysis {
			subject vehicle = vehicle_c1;
			in scenario = highwayScenario;
		}
		
		part vehicle_c1 : Vehicle {
			// ...
			
			attribute :>> fuelEconomy_city = cityAnalysis.fuelEconomyResult;
			attribute :>> fuelEconomy_highway = highwayAnalysis.fuelEconomyResult;
		}
		
		satisfy vehicleFuelEconomyRequirements by vehicle_c1;
	}

}
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "73af8304e0b36fab77f748ad6f9833ebb001f250cbe4b1833a5024f73b6a1cc3") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example"))) (kind "package") (name "Analysis Case Usage Example") (declared-name "Analysis Case Usage Example") (range (start (line 0) (character 0)) (end (line 0) (character 851))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 54))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "Analysis Case Definition Example::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 50))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (kind "part") (name "vehicleFuelEconomyAnalysisContext") (declared-name "vehicleFuelEconomyAnalysisContext") (range (start (line 3) (character 1)) (end (line 3) (character 751))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example"))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind "attribute") (name "cityScenario") (declared-name "cityScenario") (range (start (line 9) (character 2)) (end (line 9) (character 56))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "WayPoint") (range none)) (typing (reference "WayPoint") (range (start (line 9) (character 27)) (end (line 9) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind "attribute") (name "highwayScenario") (declared-name "highwayScenario") (range (start (line 10) (character 2)) (end (line 10) (character 59))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "WayPoint") (range none)) (typing (reference "WayPoint") (range (start (line 10) (character 30)) (end (line 10) (character 38)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (kind "part") (name "vehicle_c1") (declared-name "vehicle_c1") (range (start (line 22) (character 2)) (end (line 22) (character 189))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 22) (character 20)) (end (line 22) (character 27)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (kind "attribute") (name "fuelEconomy_city") (declared-name "fuelEconomy_city") (range (start (line 25) (character 3)) (end (line 25) (character 67))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_city") (range (start (line 25) (character 17)) (end (line 25) (character 33)))))))
    (element (id (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (kind "attribute") (name "fuelEconomy_highway") (declared-name "fuelEconomy_highway") (range (start (line 26) (character 3)) (end (line 26) (character 73))) (parent (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "fuelEconomy_highway") (range (start (line 26) (character 17)) (end (line 26) (character 36)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "Analysis Case Definition Example::*") (range (start (line 1) (character 16)) (end (line 1) (character 50))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (kind satisfySource) (ordinal 0)) (authored-target "vehicleFuelEconomyRequirements") (range (start (line 29) (character 10)) (end (line 29) (character 40))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (kind satisfyTarget) (ordinal 0)) (authored-target "vehicle_c1") (range (start (line 29) (character 44)) (end (line 29) (character 54))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "WayPoint") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 1)) (authored-target "WayPoint") (range (start (line 9) (character 27)) (end (line 9) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0)) (authored-target "WayPoint") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 1)) (authored-target "WayPoint") (range (start (line 10) (character 30)) (end (line 10) (character 38))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 22) (character 20)) (end (line 22) (character 27))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_city") (range (start (line 25) (character 17)) (end (line 25) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city")))))
    (reference (id (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)) (authored-target "fuelEconomy_highway") (range (start (line 26) (character 17)) (end (line 26) (character 36))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway")))))
  )
  (relationships
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (target (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))) (kind redefinition) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (target (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city")) (expression (status "unresolved") (error "expression has an unresolved reference")))
    (node (node (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway")) (expression (status "unresolved") (error "expression has an unresolved reference")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 22 20) (end 22 27)) (probe (position 22 20))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 22 20) (end 22 27))
        (outcome (status unresolved))
      )
    )
    (query (range (start 9 27) (end 9 35)) (probe (position 9 27))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))
        (kind featureTyping) (ordinal 1) (authored-target "WayPoint")
        (range (start 9 27) (end 9 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 10 30) (end 10 38)) (probe (position 10 30))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))
        (kind featureTyping) (ordinal 1) (authored-target "WayPoint")
        (range (start 10 30) (end 10 38))
        (outcome (status unresolved))
      )
    )
    (query (range (start 29 44) (end 29 54)) (probe (position 29 44))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))
        (kind satisfyTarget) (ordinal 0) (authored-target "vehicle_c1")
        (range (start 29 44) (end 29 54))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1") (range (start 22 2) (end 22 189)))
        )
      )
    )
    (query (range (start 25 17) (end 25 33)) (probe (position 25 17))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city"))
        (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_city")
        (range (start 25 17) (end 25 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_city") (range (start 25 3) (end 25 67)))
        )
      )
    )
    (query (range (start 26 17) (end 26 36)) (probe (position 26 17))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway"))
        (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_highway")
        (range (start 26 17) (end 26 36))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1::fuelEconomy_highway") (range (start 26 3) (end 26 73)))
        )
      )
    )
    (query (range (start 29 10) (end 29 40)) (probe (position 29 10))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))
        (kind satisfySource) (ordinal 0) (authored-target "vehicleFuelEconomyRequirements")
        (range (start 29 10) (end 29 40))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 50)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Analysis Case Usage Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "Analysis Case Definition Example::*")
        (range (start 1 16) (end 1 50))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
