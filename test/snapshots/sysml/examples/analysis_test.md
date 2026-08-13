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
  (document "memory://snapshot/analysis_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 3 2) (end 3 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 13 2) (end 13 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 15 2) (end 17 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 19 2) (end 19 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 23 2) (end 23 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 25 2) (end 27 3))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 29 41) (end 29 53))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 34 3) (end 34 18))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:941a96f01feb9393ead003706ede5fb4443b5801f84ff991ed3a28c779a840d6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisCase"))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisPlan"))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/analysis_test.md") (range (start 29 26) (end 29 38)) (probe (position 29 26))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))))
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 33 26) (end 33 38)) (probe (position 33 26))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))))
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 6 11) (end 6 12)) (probe (position 6 11))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
  )
)
~~~
