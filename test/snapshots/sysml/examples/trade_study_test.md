# META
~~~ini
description=SysML Example (Simple Tests): TradeStudyTest
type=file
~~~
# SOURCE
~~~sysml
package TradeStudyTest {
	private import ScalarValues::Real;
	private import TradeStudies::*;
	
	part def Engine;
	part engine1: Engine;
	part engine2: Engine;
	
	analysis engineTradeStudy : TradeStudy {
		subject : Engine[1..*] = (engine1, engine2);
		objective : MaximizeObjective;

		calc :>> evaluationFunction {
			in part : Engine;
			return : Real;
		}
		
		return part : Engine;
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/trade_study_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 16) (end 1 34))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 16) (end 2 31))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 29) (end 8 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 10 2) (end 10 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 12 11) (end 12 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 12) (end 14 16))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 23))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:82c06e70b44911029f1518c344a173cc8672463f13c53c24742fd23c580d2859") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TradeStudies") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine1"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine2"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TradeStudy"))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind calc) (ordinal 0))))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "evaluationFunction"))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind part) (ordinal 0))))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind parameter) (ordinal 0))))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TradeStudies")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine1"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine2"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind featureTyping) (ordinal 0))
      (authored-target "TradeStudy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind subject) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "evaluationFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine1"))) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine2"))) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind subject) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind part) (ordinal 0))))) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/trade_study_test.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 5 15) (end 5 21)) (probe (position 5 15))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine1"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 6 15) (end 6 21)) (probe (position 6 15))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engine2"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 8 29) (end 8 39)) (probe (position 8 29))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind featureTyping) (ordinal 0) (authored-target "TradeStudy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 9 12) (end 9 18)) (probe (position 9 12))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind subject) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 12 11) (end 12 29)) (probe (position 12 11))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "evaluationFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 13 13) (end 13 19)) (probe (position 13 13))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind part) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/trade_study_test.md") (qualified-name "TradeStudyTest::Engine")))))
  )
  (query (document "memory://snapshot/trade_study_test.md") (range (start 14 12) (end 14 16)) (probe (position 14 12))
    (reference (id (source (node (document "memory://snapshot/trade_study_test.md") (anonymous (kind parameter) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
)
~~~
