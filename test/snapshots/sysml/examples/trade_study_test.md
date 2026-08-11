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
  (document "trade_study_test.md"
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
        (range (start 2 16) (end 2 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 8 1) (end 8 227))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 2) (end 10 32))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "fb546d7f0625b2425d9b9a0fca32233ec23f04381d0a2528eaf4ead5b993bb63") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TradeStudyTest"))) (kind "package") (name "TradeStudyTest") (declared-name "TradeStudyTest"))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "TradeStudyTest"))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (kind "part") (name "engine1") (declared-name "engine1") (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (kind "part") (name "engine2") (declared-name "engine2") (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind "analysis") (name "engineTradeStudy") (declared-name "engineTradeStudy") (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (kind "analysis result") (name "") (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (kind "part") (name "") (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (authored (relationships (typing (reference "MaximizeObjective")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximizeObjective") (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (target (node (document "d0") (qualified-name "TradeStudyTest::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 15) (end 5 21)) (probe (position 5 15))
      (reference
        (source (document "d0") (qualified-name "TradeStudyTest::engine1"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 5 15) (end 5 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudyTest::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 6 15) (end 6 21)) (probe (position 6 15))
      (reference
        (source (document "d0") (qualified-name "TradeStudyTest::engine2"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 6 15) (end 6 21))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudyTest::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 13 13) (end 13 19)) (probe (position 13 13))
      (reference
        (source (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 13 13) (end 13 19))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "TradeStudyTest::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudyTest::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "TradeStudyTest::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
