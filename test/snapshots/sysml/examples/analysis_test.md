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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 19 2) (end 19 5))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness complete) (has-evaluation true) (source-digest "blake3:941a96f01feb9393ead003706ede5fb4443b5801f84ff991ed3a28c779a840d6") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (kind analysis-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (memberAccessOperand (reference "v::m")))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind requirement) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisObjective")))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (path (named (kind package) (name "AnalysisTest")) (named (kind analysis-def) (name "AnalysisCase")) (named (kind requirement) (name "obj")) (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective"))) (kind requirement-def) (membership (kind owning) (visibility default)) (documentation (doc (text " ... "))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan"))) (kind analysis-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisCase")))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase::mass"))) (kind parameter) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::objective"))) (kind requirement) (membership (kind feature) (visibility default)) (documentation (doc (text " ... "))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V::m"))) (kind default-reference) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext"))) (kind part) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "AnalysisPlan")))))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan::v"))) (kind subject) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "V")))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "v::m")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0))
      (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0))
      (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (state unsupported))
  )
)
~~~
# TYPES
~~~sexpr
(types
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))
      (subtype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))
      (type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (path (named (kind package) (name "AnalysisTest")) (named (kind analysis-def) (name "AnalysisCase")) (named (kind requirement) (name "obj")) (anonymous (kind subject) (ordinal 0)))))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))
      (type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective")))
      (subtype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))
      (subtype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))
      (type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase::mass")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::objective")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))
      (type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))
      (subtype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v")) (scopes any))
      (subtype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v")) (scopes any))
      (subtype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V::m")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext")))
      (type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")) (scopes any))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan::v")))
      (featured-by (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan")))
    )
    (declaration (id (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv")))
      (type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (provenance authored))
      (effective-type (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (source direct))
      (supertype (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")) (scopes any))
    )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/analysis_test.md") (range (start 19 2) (end 19 5)) (probe (position 19 2))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase"))) (kind memberAccessOperand) (ordinal 0) (authored-target "v::m")
      (outcome (status unresolved)))
    )
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 15 18) (end 15 35)) (probe (position 15 18))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::obj"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisObjective")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisObjective")))))
    )
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 13 14) (end 13 15)) (probe (position 13 14))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase::v"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
    )
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 29 26) (end 29 38)) (probe (position 29 26))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::analysisCase"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisCase")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisCase")))))
    )
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 23 14) (end 23 15)) (probe (position 23 14))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan::v"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
    )
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 33 26) (end 33 38)) (probe (position 33 26))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::analysisContext::analysisPlan"))) (kind featureTyping) (ordinal 0) (authored-target "AnalysisPlan")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::AnalysisPlan")))))
    )
  )
  (query (document "memory://snapshot/analysis_test.md") (range (start 6 11) (end 6 12)) (probe (position 6 11))
    (reference (id (source (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::vv"))) (kind featureTyping) (ordinal 0) (authored-target "V")
      (outcome (status resolved) (target (node (document "memory://snapshot/analysis_test.md") (qualified-name "AnalysisTest::V")))))
    )
  )
)
~~~
