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
  (document "10a_analysis.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 19))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 3 16) (end 3 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 3) (end 11 167))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 14) (end 11 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 37 17) (end 37 44))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 40 3) (end 40 28))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "65a227d0c856228b1b02f427e98671aaa90ff1584a2db7dd735198e8f966f0b8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10a-Analysis"))) (kind "package") (name "10a-Analysis") (declared-name "10a-Analysis"))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10a-Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10a-Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10a-Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (kind "package") (name "VehicleAnalysisModel") (declared-name "VehicleAnalysisModel") (parent (node (document "d0") (qualified-name "10a-Analysis"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (kind "analysis def") (name "AnalysisPlan") (declared-name "AnalysisPlan") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind "analysis") (name "massAnalysisCase") (declared-name "massAnalysisCase") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassAnalysisCase")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase::mass"))) (kind "analysis result") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind "analysis def") (name "MassAnalysisCase") (declared-name "MassAnalysisCase") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (authored (relationships (typing (reference "MassAnalysisObjective")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (authored (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind "requirement def") (name "MassAnalysisObjective") (declared-name "MassAnalysisObjective") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (authored (membership (kind Owning)) (relationships (subject (reference "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind "subject") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (authored (relationships (typing (reference "MassValue")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (kind "import") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleDesignModel::Vehicle") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (kind "part") (name "massAnalysisContext") (declared-name "massAnalysisContext") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))) (kind "package") (name "VehicleDesignModel") (declared-name "VehicleDesignModel") (parent (node (document "d0") (qualified-name "10a-Analysis"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue")) (typing (reference "MassValue")) (redefinition (reference "mass")))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAnalysisCase") (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAnalysisObjective") (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "VehicleDesignModel::Vehicle") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind referenceSubsetting) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind referenceSubsetting) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind redefinition) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass")) (expression (status "unsupported") (error "declared expression form is not supported")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 2 16) (end 2 18)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "10a-Analysis::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "SI::*")
        (range (start 2 16) (end 2 18))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 19)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "10a-Analysis::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "ISQ::*")
        (range (start 1 16) (end 1 19))
        (outcome (status unresolved))
      )
    )
    (query (range (start 11 3) (end 11 11)) (probe (position 11 3))
      (reference
        (source (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))
        (kind redefinition) (ordinal 0) (authored-target "mass")
        (range (start 11 3) (end 11 11))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass") (range (start 11 3) (end 11 167)))
        )
      )
    )
    (query (range (start 11 14) (end 11 23)) (probe (position 11 14))
      (reference
        (source (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))
        (kind featureTyping) (ordinal 1) (authored-target "MassValue")
        (range (start 11 14) (end 11 23))
        (outcome (status unresolved))
      )
    )
    (query (range (start 3 16) (end 3 34)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "10a-Analysis::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "NumericalFunctions::*")
        (range (start 3 16) (end 3 34))
        (outcome (status unresolved))
      )
    )
    (query (range (start 37 17) (end 37 44)) (probe (position 37 17))
      (reference
        (source (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))
        (kind membershipImport) (ordinal 0) (authored-target "VehicleDesignModel::Vehicle")
        (range (start 37 17) (end 37 44))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
