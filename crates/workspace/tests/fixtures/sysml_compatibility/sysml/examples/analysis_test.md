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
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "AnalysisTest"))) (name "AnalysisTest") (declared-name "AnalysisTest")
      (contains
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (name "AnalysisCase") (declared-name "AnalysisCase")
          (contains
            (element (kind "objective") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (name "obj") (declared-name "obj") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase")))))
          )
        )
        (element (kind "requirement def") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective"))) (name "AnalysisObjective") (declared-name "AnalysisObjective")
          (contains
            (element (kind "documentation") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective")))))
          )
        )
        (element (kind "analysis def") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))) (name "AnalysisPlan") (declared-name "AnalysisPlan")
          (contains
            (element (kind "analysis") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (name "analysisCase") (declared-name "analysisCase") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))))
              (contains
                (element (kind "analysis result") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase")))))
              )
            )
            (element (kind "objective") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::objective"))) (name "objective") (declared-name "objective") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan")))))
            (element (kind "subject") (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (name "v") (declared-name "v") (effective (featuring-type (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "AnalysisTest::V"))) (name "V") (declared-name "V") (declared))
        (element (kind "part") (id (node (document "d0") (qualified-name "AnalysisTest::analysisContext"))) (name "analysisContext") (declared-name "analysisContext") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "AnalysisTest::vv"))) (name "vv") (declared-name "vv") (declared (properties (composite true) (reference false) (ordered false))))
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective::_documentation"))) (to (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (to (node (document "d0") (qualified-name "AnalysisTest::V"))))
    (subject (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))) (to (node (document "d0") (qualified-name "AnalysisTest::V"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (to (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::v"))) (to (node (document "d0") (qualified-name "AnalysisTest::V"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (to (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (to (node (document "d0") (qualified-name "AnalysisTest::V"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "AnalysisTest::vv"))) (to (node (document "d0") (qualified-name "AnalysisTest::V"))))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/examples/analysis_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 3 2) (end 3 6))
      )
      (diagnostic
        (severity warning)
        (code "case_objective_binding_cardinality")
        (source "semantic")
        (range (start 12 1) (end 12 124))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 15 2) (end 15 63))
      )
      (diagnostic
        (severity warning)
        (code "case_objective_binding_cardinality")
        (source "semantic")
        (range (start 22 1) (end 22 145))
      )
      (diagnostic
        (severity warning)
        (code "objective_binding_unresolved")
        (source "semantic")
        (range (start 25 2) (end 25 34))
      )
    )
  )
)
~~~
