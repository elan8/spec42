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
            = vehicle.mass;
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
(model
  (namespace
    (package '10a-Analysis'
      (namespace_import private -> 'ISQ'[unresolved])
      (namespace_import private -> 'SI'[unresolved])
      (namespace_import private -> 'NumericalFunctions'[unresolved])
      (package 'VehicleDesignModel'
        (part_def 'Vehicle'
          (reference_usage reference 'mass' : 'MassValue'[unresolved]))
        (part_usage 'vehicle'
          (reference_usage reference :>> 'mass'[unresolved] : 'MassValue'[unresolved]
            (feature_value (=)))
          (part_usage composite 'engine'
            (reference_usage reference 'mass' : 'MassValue'[unresolved]))
          (part_usage composite 'transmission'
            (reference_usage reference 'mass' : 'MassValue'[unresolved]))
          (part_usage composite 'frontAxleAssembly'
            (reference_usage reference 'mass' : 'MassValue'[unresolved]))
          (part_usage composite 'rearAxleAssembly'
            (reference_usage reference 'mass' : 'MassValue'[unresolved]))))
      (package 'VehicleAnalysisModel'
        (membership_import private -> '10a-Analysis::VehicleDesignModel::Vehicle'[part_def])
        (requirement_def 'MassAnalysisObjective'
          (subject_membership in 'mass' : 'MassValue'[unresolved])
          (documentation))
        (analysis_case_def 'MassAnalysisCase'
          (subject_membership in 'vehicle' : '10a-Analysis::VehicleDesignModel::Vehicle'[part_def])
          (objective_membership composite : '10a-Analysis::VehicleAnalysisModel::MassAnalysisObjective'[requirement_def]
            (subject_membership in
              (feature_value (=))))
          (result_expr_membership))
        (analysis_case_def 'AnalysisPlan'
          (subject_membership in 'vehicle' : '10a-Analysis::VehicleDesignModel::Vehicle'[part_def])
          (objective_membership composite
            (documentation))
          (analysis_case_usage composite 'massAnalysisCase' : '10a-Analysis::VehicleAnalysisModel::MassAnalysisCase'[analysis_case_def]
            (return_parameter_membership
              (feature_def out 'mass'))))
        (part_usage 'massAnalysisContext'
          (analysis_case_usage composite 'analysisPlan' : '10a-Analysis::VehicleAnalysisModel::AnalysisPlan'[analysis_case_def]
            (subject_membership in 'vehicle'
              (feature_value (=)))))))))
~~~
