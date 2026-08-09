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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
ColonGtGt,Ident,Colon,Ident,Eq,Ident,OpenParen,OpenParen,
Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,Comma,
Ident,Dot,Ident,Dot,Ident,
CloseParen,CloseParen,Semicolon,
KwPart,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,OpenCurly,
Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwRequirement,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwDoc,RegularComment,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,Colon,Ident,OpenCurly,
KwSubject,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
LineComment,
Ident,Dot,Ident,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
RegularComment,
KwReturn,Ident,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,ColonColon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''10a-Analysis''
    (import_decl private 'ISQ::*')
    (import_decl private 'SI::*')
    (import_decl private 'NumericalFunctions::*')
    (package_def 'VehicleDesignModel'
      (part_def 'Vehicle'
        (default_ref_usage 'mass' : 'MassValue'))
      (part_usage 'vehicle'
        (default_ref_usage :>> 'mass' : 'MassValue' value)
        (part_usage 'engine'
          (default_ref_usage 'mass' : 'MassValue'))
        (part_usage 'transmission'
          (default_ref_usage 'mass' : 'MassValue'))
        (part_usage 'frontAxleAssembly'
          (default_ref_usage 'mass' : 'MassValue'))
        (part_usage 'rearAxleAssembly'
          (default_ref_usage 'mass' : 'MassValue'))))
    (package_def 'VehicleAnalysisModel'
      (import_decl private 'VehicleDesignModel::Vehicle')
      (requirement_def 'MassAnalysisObjective'
        (sysml_decl 'mass' : 'MassValue')
        (documentation))
      (analysis_case_def 'MassAnalysisCase'
        (sysml_decl 'vehicle' : 'Vehicle')
        (objective_member)
        (line_comment)
        (result_expr_member))
      (analysis_case_def 'AnalysisPlan'
        (sysml_decl 'vehicle' : 'Vehicle')
        (objective_member)
        (sysml_decl 'massAnalysisCase' : 'MassAnalysisCase'
          (comment)
          (return_member)))
      (part_usage 'massAnalysisContext'
        (sysml_decl 'analysisPlan' : 'AnalysisPlan'
          (sysml_decl 'vehicle' value))))))
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
# EXPECTED
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'mass'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
semantic.unresolved_name 'MassValue'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "10a-Analysis"))) (name "10a-Analysis") (declared-name "10a-Analysis")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "10a-Analysis::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10a-Analysis::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10a-Analysis::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel"))) (name "VehicleAnalysisModel") (declared-name "VehicleAnalysisModel")
          (contains
            (element (kind "analysis def") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (name "AnalysisPlan") (declared-name "AnalysisPlan")
              (contains
                (element (kind "analysis") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (name "massAnalysisCase") (declared-name "massAnalysisCase") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))))
                  (contains
                    (element (kind "analysis result") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
                  )
                )
                (element (kind "objective") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan")))))
              )
            )
            (element (kind "analysis def") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (name "MassAnalysisCase") (declared-name "MassAnalysisCase")
              (contains
                (element (kind "objective") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (name "vehicle") (declared-name "vehicle") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase")))))
              )
            )
            (element (kind "requirement def") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (name "MassAnalysisObjective") (declared-name "MassAnalysisObjective")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))))
                (element (kind "subject") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective")))))
              )
            )
            (element (kind "import") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle"))
            (element (kind "part") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (name "massAnalysisContext") (declared-name "massAnalysisContext") (declared (properties (ordered false))))
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel"))) (name "VehicleDesignModel") (declared-name "VehicleDesignModel")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
            (element (kind "part") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (name "frontAxleAssembly") (declared-name "frontAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "attribute") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (name "mass") (declared-name "mass") (declared (properties (ordered false) (unique true)) (feature-value (kind bound) (expression (kind "invocation") (children (expression (kind "featureReference") (reference "sum"))) (arguments (argument (expression (kind "tuple") (children (expression (kind "memberAccess") (reference "mass") (children (expression (kind "memberAccess") (reference "engine") (children (expression (kind "featureReference") (reference "vehicle")))))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "memberAccess") (reference "transmission") (children (expression (kind "featureReference") (reference "vehicle")))))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "memberAccess") (reference "frontAxleAssembly") (children (expression (kind "featureReference") (reference "vehicle")))))) (expression (kind "memberAccess") (reference "mass") (children (expression (kind "memberAccess") (reference "rearAxleAssembly") (children (expression (kind "featureReference") (reference "vehicle"))))))))))))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (implied-feature-value-binding (owner (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (role feature-value))) (evaluation (expression (status "unsupported") (error "declared expression form is not supported"))))
                (element (kind "part") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (name "rearAxleAssembly") (declared-name "rearAxleAssembly") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                (element (kind "part") (id (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (name "transmission") (declared-name "transmission") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::_documentation"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective::mass"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::vehicle"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::vehicle"))) (to (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan"))) (status missing-prerequisite) (target "AnalysisCases::AnalysisCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::massAnalysisCase"))) (status missing-prerequisite) (target "AnalysisCases::analysisCases"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::AnalysisPlan::objective"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase"))) (status missing-prerequisite) (target "AnalysisCases::AnalysisCase"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisCase::objective"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective"))) (status missing-prerequisite) (target "Requirements::RequirementCheck"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleAnalysisModel::massAnalysisContext"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::frontAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::mass"))) (status missing-prerequisite) (target "Base::dataValues"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::rearAxleAssembly"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10a-Analysis::VehicleDesignModel::vehicle::transmission"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/10a_analysis.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 11 3) (end 11 167))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 3) (end 11 167))
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
      (diagnostic
        (severity warning)
        (code "case_objective_binding_cardinality")
        (source "semantic")
        (range (start 44 2) (end 44 187))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 46 3) (end 46 86))
      )
      (diagnostic
        (severity warning)
        (code "case_objective_binding_cardinality")
        (source "semantic")
        (range (start 54 2) (end 54 337))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 56 3) (end 56 37))
      )
    )
  )
)
~~~
