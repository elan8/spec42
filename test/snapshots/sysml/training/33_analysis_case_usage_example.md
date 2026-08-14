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
  (document "memory://snapshot/33_analysis_case_usage_example.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 53))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 5 21) (end 5 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 27) (end 9 35))
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
        (range (start 12 26) (end 12 45))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 14 3) (end 14 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 29) (end 17 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 19 3) (end 19 33))
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
        (range (start 25 17) (end 25 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 25 36) (end 25 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 17) (end 26 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 39) (end 26 72))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:921e1c9568d80f2dd3752af04209c82f2ccd10ef98474fd5daa6e6ac482f3b22") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Analysis Case Definition Example") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfy) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (satisfySource (reference "vehicleFuelEconomyRequirements")) (satisfyTarget (reference "vehicle_c1"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyAnalysis"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WayPoint"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayAnalysis"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "FuelEconomyAnalysis"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayAnalysis::vehicle"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind attribute) (membership (kind feature) (visibility default)) (facts (multiplicity (lower unbounded) (upper unbounded))) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "WayPoint"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements"))) (kind requirement) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelEconomy_city")) (memberAccessOperand (reference "cityAnalysis::fuelEconomyResult"))))
    (declaration (id (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "fuelEconomy_highway")) (memberAccessOperand (reference "highwayAnalysis::fuelEconomyResult"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Analysis Case Definition Example")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0))
      (authored-target "vehicleFuelEconomyRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0))
      (authored-target "vehicle_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1")))))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyAnalysis")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "WayPoint")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayAnalysis"))) (kind featureTyping) (ordinal 0))
      (authored-target "FuelEconomyAnalysis")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0))
      (authored-target "WayPoint")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelEconomy_city")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0))
      (authored-target "fuelEconomy_highway")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "cityAnalysis::fuelEconomyResult")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "highwayAnalysis::fuelEconomyResult")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind satisfySource) (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0)))
    (relationship (kind satisfyTarget) (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (target (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 1 16) (end 1 53)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "Analysis Case Definition Example")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 29 10) (end 29 40)) (probe (position 29 10))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfySource) (ordinal 0) (authored-target "vehicleFuelEconomyRequirements")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements")))))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 29 44) (end 29 54)) (probe (position 29 44))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind satisfy) (ordinal 0))))) (kind satisfyTarget) (ordinal 0) (authored-target "vehicle_c1")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1")))))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 12 26) (end 12 45)) (probe (position 12 26))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyAnalysis")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 9 27) (end 9 35)) (probe (position 9 27))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::cityScenario"))) (kind featureTyping) (ordinal 0) (authored-target "WayPoint")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 17 29) (end 17 48)) (probe (position 17 29))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayAnalysis"))) (kind featureTyping) (ordinal 0) (authored-target "FuelEconomyAnalysis")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 10 30) (end 10 38)) (probe (position 10 30))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::highwayScenario"))) (kind featureTyping) (ordinal 0) (authored-target "WayPoint")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 5 21) (end 5 28)) (probe (position 5 21))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicleFuelEconomyRequirements::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 22 20) (end 22 27)) (probe (position 22 20))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (qualified-name "Analysis Case Usage Example::vehicleFuelEconomyAnalysisContext::vehicle_c1"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 25 17) (end 25 33)) (probe (position 25 17))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_city")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 26 17) (end 26 36)) (probe (position 26 17))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind redefinition) (ordinal 0) (authored-target "fuelEconomy_highway")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 25 36) (end 25 66)) (probe (position 25 36))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "cityAnalysis::fuelEconomyResult")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_analysis_case_usage_example.md") (range (start 26 39) (end 26 72)) (probe (position 26 39))
    (reference (id (source (node (document "memory://snapshot/33_analysis_case_usage_example.md") (anonymous (kind attribute) (ordinal 1))))) (kind memberAccessOperand) (ordinal 0) (authored-target "highwayAnalysis::fuelEconomyResult")
      (outcome (status unresolved)))
  )
)
~~~
