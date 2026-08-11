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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "65a227d0c856228b1b02f427e98671aaa90ff1584a2db7dd735198e8f966f0b8") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10a-Analysis"))) (kind "package") (name "10a-Analysis") (declared-name "10a-Analysis") (range (start (line 0) (character 0)) (end (line 0) (character 1450))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 1) (character 1)) (end (line 1) (character 23))) (parent (node (document "d0") (qualified-name "10a-Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "ISQ::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 19))))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 22))) (parent (node (document "d0") (qualified-name "10a-Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "SI::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 18))))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 38))) (parent (node (document "d0") (qualified-name "10a-Analysis"))) (authored (membership (kind Import) (visibility "private") (import (reference "NumericalFunctions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 34))))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (kind "package") (name "VehicleAnalysisModel") (declared-name "VehicleAnalysisModel") (range (start (line 36) (character 1)) (end (line 36) (character 840))) (parent (node (document "d0") (qualified-name "10a-Analysis"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (kind "analysis def") (name "AnalysisPlan") (declared-name "AnalysisPlan") (range (start (line 54) (character 2)) (end (line 54) (character 337))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind "analysis") (name "massAnalysisCase") (declared-name "massAnalysisCase") (range (start (line 60) (character 3)) (end (line 60) (character 228))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassAnalysisCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase::mass"))) (kind "analysis result") (name "mass") (declared-name "mass") (range (start (line 65) (character 5)) (end (line 65) (character 17))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 56) (character 3)) (end (line 56) (character 37))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 55) (character 3)) (end (line 55) (character 29))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (kind "analysis def") (name "MassAnalysisCase") (declared-name "MassAnalysisCase") (range (start (line 44) (character 2)) (end (line 44) (character 187))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 46) (character 3)) (end (line 46) (character 86))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (authored (relationships (typing (reference "MassAnalysisObjective") (range none)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind "subject") (name "vehicle") (declared-name "vehicle") (range (start (line 45) (character 3)) (end (line 45) (character 29))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (authored (relationships (typing (reference "Vehicle") (range none)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind "requirement def") (name "MassAnalysisObjective") (declared-name "MassAnalysisObjective") (range (start (line 39) (character 2)) (end (line 39) (character 91))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (authored (membership (kind Owning)) (relationships (subject (reference "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::_documentation"))) (kind "documentation") (name "") (range (start (line 39) (character 2)) (end (line 39) (character 91))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind "subject") (name "mass") (declared-name "mass") (range (start (line 40) (character 3)) (end (line 40) (character 28))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (authored (relationships (typing (reference "MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (kind "import") (name "Vehicle") (declared-name "Vehicle") (range (start (line 37) (character 2)) (end (line 37) (character 45))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (authored (membership (kind Import) (visibility "private") (import (reference "VehicleDesignModel::Vehicle") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 37) (character 17)) (end (line 37) (character 44))))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (kind "part") (name "massAnalysisContext") (declared-name "massAnalysisContext") (range (start (line 69) (character 2)) (end (line 69) (character 130))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))) (kind "package") (name "VehicleDesignModel") (declared-name "VehicleDesignModel") (range (start (line 5) (character 1)) (end (line 5) (character 492))) (parent (node (document "d0") (qualified-name "10a-Analysis"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 6) (character 2)) (end (line 6) (character 45))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 10) (character 2)) (end (line 10) (character 410))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 18) (character 3)) (end (line 18) (character 43))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (kind "part") (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (range (start (line 26) (character 3)) (end (line 26) (character 54))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind "attribute") (name "mass") (declared-name "mass") (range (start (line 11) (character 3)) (end (line 11) (character 167))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassValue") (range none)) (typing (reference "MassValue") (range (start (line 11) (character 14)) (end (line 11) (character 23)))) (redefinition (reference "mass") (range (start (line 11) (character 3)) (end (line 11) (character 11)))))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (kind "part") (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (range (start (line 30) (character 3)) (end (line 30) (character 53))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
    (element (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (kind "part") (name "transmission") (declared-name "transmission") (range (start (line 22) (character 3)) (end (line 22) (character 52))) (parent (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "ISQ::*") (range (start (line 1) (character 16)) (end (line 1) (character 19))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "SI::*") (range (start (line 2) (character 16)) (end (line 2) (character 18))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "NumericalFunctions::*") (range (start (line 3) (character 16)) (end (line 3) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAnalysisCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MassAnalysisObjective") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (kind referenceSubsetting) (ordinal 0)) (authored-target "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass")))))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (kind membershipImport) (ordinal 0)) (authored-target "VehicleDesignModel::Vehicle") (range (start (line 37) (character 17)) (end (line 37) (character 44))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind featureTyping) (ordinal 1)) (authored-target "MassValue") (range (start (line 11) (character 14)) (end (line 11) (character 23))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (kind redefinition) (ordinal 0)) (authored-target "mass") (range (start (line 11) (character 3)) (end (line 11) (character 11))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass")))))
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
