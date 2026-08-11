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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "analysis_test.md"
    (diagnostics
      (diagnostic
        (severity error)
        (code "unrecognized_declaration_in_scope")
        (source "sysml")
        (range (start 3 2) (end 3 6))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness editor-recovery) (has-evaluation true) (source-digest "f5930d3b5479c0fb9542f59bb5a8de20e48107aa6850cf49b94253e48a87b265") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "AnalysisTest"))) (kind "package") (name "AnalysisTest") (declared-name "AnalysisTest") (range (start (line 0) (character 0)) (end (line 0) (character 488))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (kind "analysis def") (name "AnalysisCase") (declared-name "AnalysisCase") (range (start (line 12) (character 1)) (end (line 12) (character 124))) (parent (node (document "d0") (qualified-name "AnalysisTest"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind "objective") (name "obj") (declared-name "obj") (range (start (line 15) (character 2)) (end (line 15) (character 63))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (authored (relationships (typing (reference "AnalysisObjective") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind "subject") (name "v") (declared-name "v") (range (start (line 13) (character 2)) (end (line 13) (character 16))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (authored (relationships (typing (reference "V") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective"))) (kind "requirement def") (name "AnalysisObjective") (declared-name "AnalysisObjective") (range (start (line 8) (character 1)) (end (line 8) (character 55))) (parent (node (document "d0") (qualified-name "AnalysisTest"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective::_documentation"))) (kind "documentation") (name "") (range (start (line 8) (character 1)) (end (line 8) (character 55))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))) (kind "analysis def") (name "AnalysisPlan") (declared-name "AnalysisPlan") (range (start (line 22) (character 1)) (end (line 22) (character 145))) (parent (node (document "d0") (qualified-name "AnalysisTest"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind "analysis") (name "analysisCase") (declared-name "analysisCase") (range (start (line 29) (character 2)) (end (line 29) (character 55))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))) (authored (membership (kind Feature)) (relationships (typing (reference "AnalysisCase") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase::mass"))) (kind "analysis result") (name "mass") (declared-name "mass") (range (start (line 29) (character 41)) (end (line 29) (character 53))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 25) (character 2)) (end (line 25) (character 34))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind "subject") (name "v") (declared-name "v") (range (start (line 23) (character 2)) (end (line 23) (character 16))) (parent (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))) (authored (relationships (typing (reference "V") (range none)))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::V"))) (kind "part def") (name "V") (declared-name "V") (range (start (line 2) (character 1)) (end (line 2) (character 21))) (parent (node (document "d0") (qualified-name "AnalysisTest"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::analysisContext"))) (kind "part") (name "analysisContext") (declared-name "analysisContext") (range (start (line 32) (character 1)) (end (line 32) (character 90))) (parent (node (document "d0") (qualified-name "AnalysisTest"))))
    (element (id (node (document "d0") (qualified-name "AnalysisTest::vv"))) (kind "part") (name "vv") (declared-name "vv") (range (start (line 6) (character 1)) (end (line 6) (character 13))) (parent (node (document "d0") (qualified-name "AnalysisTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "V") (range (start (line 6) (character 11)) (end (line 6) (character 12)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisObjective") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0)) (authored-target "AnalysisCase") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisTest::V")))))
    (reference (id (source (node (document "d0") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0)) (authored-target "V") (range (start (line 6) (character 11)) (end (line 6) (character 12))) (outcome (status resolved) (target (node (document "d0") (qualified-name "AnalysisTest::V")))))
  )
  (relationships
    (relationship (kind subject) (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (target (node (document "d0") (qualified-name "AnalysisTest::V"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (target (node (document "d0") (qualified-name "AnalysisTest::AnalysisObjective"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::v"))) (target (node (document "d0") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan"))) (target (node (document "d0") (qualified-name "AnalysisTest::V"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (target (node (document "d0") (qualified-name "AnalysisTest::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (target (node (document "d0") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "AnalysisTest::vv"))) (target (node (document "d0") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 6 11) (end 6 12)) (probe (position 6 11))
      (reference
        (source (document "d0") (qualified-name "AnalysisTest::vv"))
        (kind featureTyping) (ordinal 0) (authored-target "V")
        (range (start 6 11) (end 6 12))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "AnalysisTest::V") (range (start 2 1) (end 2 21)))
        )
      )
    )
  )
)
~~~
