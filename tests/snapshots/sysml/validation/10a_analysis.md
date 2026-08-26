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
        (severity information)
        (code "missing_library_context")
        (source "semantic")
        (range (start 1 16) (end 1 22))
      )
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
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 7 10) (end 7 19))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 10 2) (end 33 3))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 11 26) (end 11 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 4) (end 12 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 13 4) (end 13 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 14 4) (end 14 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 15 4) (end 15 33))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 18 3) (end 20 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 19 11) (end 19 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 22 3) (end 24 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 23 14) (end 23 23))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 26 3) (end 28 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 27 11) (end 27 20))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 30 3) (end 32 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 31 11) (end 31 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 18) (end 40 27))
      )
      (diagnostic
        (severity information)
        (code "untyped_part_usage")
        (source "semantic")
        (range (start 69 2) (end 73 3))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:f2396f6496b97f2feca2ddba2d175a44d6ece1086a2d61db58b234e5f33cc103"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "ISQ") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "SI") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "NumericalFunctions") (import (shape namespace) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "VehicleDesignModel::Vehicle") (import (shape membership) (recursive false))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind analysis) (membership (kind feature) (visibility default)) (documentation (comment (text "\n\t\t\t\t * By default, the subject of a nested analysis case bound to that\n\t\t\t\t * of its containing analysis case or analysis case definition.\n\t\t\t\t "))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassAnalysisCase")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase::mass"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " ... "))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind analysis-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::mass")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassAnalysisObjective")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " ... "))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisPlan")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan::vehicle"))) (kind subject) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0))))) (kind attribute) (membership (kind feature) (visibility default)) (feature-value (kind bind) (value (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (result (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")) (redefinition (reference "mass")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind kerml-expression) (membership (kind owning) (visibility default)) (facts (expression-result (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "vehicle::engine::mass")) (memberAccessOperand (reference "vehicle::transmission::mass")) (memberAccessOperand (reference "vehicle::frontAxleAssembly::mass")) (memberAccessOperand (reference "vehicle::rearAxleAssembly::mass")) (invocationCallee (reference "sum")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (kind kerml-feature) (membership (kind feature) (visibility default)) (facts (direction out)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine::mass"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly::mass"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly::mass"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission::mass"))) (kind default-reference) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassValue")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0))
      (authored-target "ISQ")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "SI")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "VehicleDesignModel::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassAnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassAnalysisObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "vehicle::engine::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "vehicle::transmission::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 2))
      (authored-target "vehicle::frontAxleAssembly::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 3))
      (authored-target "vehicle::rearAxleAssembly::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0))
      (authored-target "sum")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassValue")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind memberAccessOperand) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind memberAccessOperand) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan::vehicle"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (provenance implied))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan::vehicle"))) (target (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0))))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0))))) (target (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (provenance implied))
    (relationship (kind typeFeaturing) (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission::mass"))) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (provenance implied))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (state unsupported))
    (evaluated (declaration (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))
      (type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")) (source direct))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))
      (type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))
      (type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")) (source direct))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective")))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind analysis-def) (name "MassAnalysisCase")) (named (kind requirement) (name "objective")) (anonymous (kind subject) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))
      (type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")) (source direct))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext")))
      (type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")) (provenance authored))
      (effective-type (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")) (source direct))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan::vehicle")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan")))
      (supertype (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (named (kind part) (name "massAnalysisContext")) (named (kind analysis) (name "analysisPlan")) (named (kind subject) (name "vehicle")) (anonymous (kind kerml-expression) (ordinal 0)))))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan::vehicle")) (scopes any feature))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle")) (scopes any))
      (subtype (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)) (anonymous (kind kerml-feature) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0)))))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle")))
    )
    (declaration (id (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission::mass")))
      (featured-by (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission")))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/10a_analysis.md") (range (start 1 16) (end 1 22)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 0))))) (kind namespaceImport) (ordinal 0) (authored-target "ISQ")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 2 16) (end 2 21)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "SI")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 3 16) (end 3 37)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 37 17) (end 37 44)) (probe (position 37 17))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleAnalysisModel")) (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "VehicleDesignModel::Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 60 31) (end 60 47)) (probe (position 60 31))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0) (authored-target "MassAnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 55 21) (end 55 28)) (probe (position 55 21))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 51 3) (end 51 15)) (probe (position 51 3))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::mass")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 46 15) (end 46 36)) (probe (position 46 15))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind featureTyping) (ordinal 0) (authored-target "MassAnalysisObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 45 21) (end 45 28)) (probe (position 45 21))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 40 18) (end 40 27)) (probe (position 40 18))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 70 27) (end 70 39)) (probe (position 70 27))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 7 10) (end 7 19)) (probe (position 7 10))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 11 14) (end 11 23)) (probe (position 11 14))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 11 7) (end 11 11)) (probe (position 11 7))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 12 4) (end 12 23)) (probe (position 12 4))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 0) (authored-target "vehicle::engine::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 13 4) (end 13 29)) (probe (position 13 4))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 1) (authored-target "vehicle::transmission::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 14 4) (end 14 34)) (probe (position 14 4))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 2) (authored-target "vehicle::frontAxleAssembly::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 15 4) (end 15 33)) (probe (position 15 4))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind memberAccessOperand) (ordinal 3) (authored-target "vehicle::rearAxleAssembly::mass")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 11 26) (end 11 29)) (probe (position 11 26))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (path (named (kind package) (name "10a-Analysis")) (named (kind package) (name "VehicleDesignModel")) (named (kind part) (name "vehicle")) (anonymous (kind attribute) (ordinal 0)) (anonymous (kind kerml-expression) (ordinal 0))))) (kind invocationCallee) (ordinal 0) (authored-target "sum")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 19 11) (end 19 20)) (probe (position 19 11))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 27 11) (end 27 20)) (probe (position 27 11))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 31 11) (end 31 20)) (probe (position 31 11))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/10a_analysis.md") (range (start 23 14) (end 23 23)) (probe (position 23 14))
    (reference (id (source (node (document "memory://snapshot/10a_analysis.md") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission::mass"))) (kind featureTyping) (ordinal 0) (authored-target "MassValue")
      (outcome (status unresolved)))
    )
  )
)
~~~
