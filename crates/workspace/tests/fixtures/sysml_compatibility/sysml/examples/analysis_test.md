# META
~~~ini
description=SysML Example (Simple Tests): AnalysisTest
type=file
~~~
# SOURCE
~~~sysml
package AnalysisTest {

	part def V {
		m;
	}
	
	part vv : V;
	
	requirement def AnalysisObjective {
		doc /* ... */
	}

	analysis def AnalysisCase {
		subject v : V;
		
		objective obj : AnalysisObjective { 
			subject = result;
		}
		
		v.m
	}
	
	analysis def AnalysisPlan {
		subject v : V;
		
		objective {
			doc /* ... */
		}
		
		analysis analysisCase : AnalysisCase { return mass; }
	}
	
	part analysisContext {
		analysis analysisPlan : AnalysisPlan {
			subject v = vv;
		}
	}
}
~~~
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,OpenCurly,
Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
KwRequirement,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,Ident,Colon,Ident,OpenCurly,
KwSubject,Eq,Ident,Semicolon,
CloseCurly,
Ident,Dot,Ident,
CloseCurly,
KwAnalysis,KwDef,Ident,OpenCurly,
KwSubject,Ident,Colon,Ident,Semicolon,
KwObjective,OpenCurly,
KwDoc,RegularComment,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,KwReturn,Ident,Semicolon,CloseCurly,
CloseCurly,
KwPart,Ident,OpenCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Ident,Eq,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'AnalysisTest'
    (part_def 'V'
      (default_ref_usage 'm'))
    (part_usage 'vv' : 'V')
    (requirement_def 'AnalysisObjective'
      (documentation))
    (analysis_case_def 'AnalysisCase'
      (sysml_decl 'v' : 'V')
      (objective_member)
      (result_expr_member))
    (analysis_case_def 'AnalysisPlan'
      (sysml_decl 'v' : 'V')
      (objective_member)
      (sysml_decl 'analysisCase' : 'AnalysisCase'
        (return_member)))
    (part_usage 'analysisContext'
      (sysml_decl 'analysisPlan' : 'AnalysisPlan'
        (sysml_decl 'v' value)))))
~~~
# FORMAT
~~~sysml
package AnalysisTest {
    part def V {
        m;
    }

    part vv : V;

    requirement def AnalysisObjective {
        doc /* ... */
    }

    analysis def AnalysisCase {
        subject v : V;

        objective obj : AnalysisObjective {
            subject = result;
        }

        = v.m;
    }

    analysis def AnalysisPlan {
        subject v : V;

        objective {
            doc /* ... */
        }

        analysis analysisCase : AnalysisCase {
            return mass;
        }
    }

    part analysisContext {
        analysis analysisPlan : AnalysisPlan {
            subject v = vv;
        }
    }
}
~~~
# EXPECTED
~~~
NIL
~~~
# PROBLEMS
~~~
NIL
~~~
# SMG
~~~
(model
  (namespace
    (package 'AnalysisTest'
      (part_def 'V'
        (reference_usage reference 'm'))
      (part_usage 'vv' : 'AnalysisTest::V'[part_def])
      (requirement_def 'AnalysisObjective'
        (documentation))
      (analysis_case_def 'AnalysisCase'
        (subject_membership in 'v' : 'AnalysisTest::V'[part_def])
        (objective_membership composite 'obj' : 'AnalysisTest::AnalysisObjective'[requirement_def]
          (subject_membership in
            (feature_value (=))))
        (result_expr_membership))
      (analysis_case_def 'AnalysisPlan'
        (subject_membership in 'v' : 'AnalysisTest::V'[part_def])
        (objective_membership composite
          (documentation))
        (analysis_case_usage composite 'analysisCase' : 'AnalysisTest::AnalysisCase'[analysis_case_def]
          (return_parameter_membership
            (feature_def out 'mass'))))
      (part_usage 'analysisContext'
        (analysis_case_usage composite 'analysisPlan' : 'AnalysisTest::AnalysisPlan'[analysis_case_def]
          (subject_membership in 'v'
            (feature_value (=))))))))
~~~
