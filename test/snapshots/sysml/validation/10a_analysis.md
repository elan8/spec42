# META
~~~ini
description=SysML Validation (10-Analysis and Trades): 10a-Analysis
type=file
~~~
# SOURCE
~~~sysml
package '10a-Analysis' {
	private import ISQ::*;
	private import SI::*;
	private import NumericalFunctions::*;
	
	package VehicleDesignModel {
		part def Vehicle {
			mass : MassValue;
		}
		
		part vehicle {
			:>> mass : MassValue = sum((
				vehicle.engine.mass, 
				vehicle.transmission.mass, 
				vehicle.frontAxleAssembly.mass, 
				vehicle.rearAxleAssembly.mass
			));
			
			part engine {
				mass : MassValue;
			}
			
			part transmission {
			    mass : MassValue;
			}
			
			part frontAxleAssembly {
				mass : MassValue;
			}
			
			part rearAxleAssembly {
				mass : MassValue;
			}
		}
	}
	
	package VehicleAnalysisModel {
		private import VehicleDesignModel::Vehicle;
		
		requirement def MassAnalysisObjective {
			subject mass : MassValue;
			doc /* ... */
		}
	
		analysis def MassAnalysisCase {
			subject vehicle : Vehicle;
			objective : MassAnalysisObjective {
			    subject = MassAnalysisCase::result;
			}
			
			// Result
			vehicle.mass
		}
		
		analysis def AnalysisPlan {
			subject vehicle : Vehicle;			
			objective {
				doc /* ... */
			}
			
			analysis massAnalysisCase : MassAnalysisCase {
				/*
				 * By default, the subject of a nested analysis case bound to that
				 * of its containing analysis case or analysis case definition.
				 */
			 	return mass; 
			 }
		}
		
		part massAnalysisContext {
			analysis analysisPlan : AnalysisPlan {
				subject vehicle = VehicleDesignModel::vehicle;
			}
		}
	}
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/10a_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 21))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 37))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 7 3) (end 7 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 7) (end 11 11))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 14) (end 11 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 19 4) (end 19 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 23 7) (end 23 24))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 27 4) (end 27 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 31 4) (end 31 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_requirement_definition_member")
        (source "semantic")
        (range (start 40 3) (end 40 28))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 45 3) (end 45 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 46 3) (end 48 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 51 3) (end 51 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 55 3) (end 55 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 56 3) (end 58 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 65 5) (end 65 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 71 4) (end 71 50))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:f2396f6496b97f2feca2ddba2d175a44d6ece1086a2d61db58b234e5f33cc103") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VehicleDesignModel::Vehicle") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassAnalysisCase"))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisPlan"))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "mass"))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleDesignModel::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassAnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/10a_analysis.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 37 17) (end 37 44)) (probe (position 37 17))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleDesignModel::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 60 31) (end 60 47)) (probe (position 60 31))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0) (authored-target "MassAnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 70 27) (end 70 39)) (probe (position 70 27))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 11 14) (end 11 23)) (probe (position 11 14))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 11 7) (end 11 11)) (probe (position 11 7))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
  )
)
~~~
