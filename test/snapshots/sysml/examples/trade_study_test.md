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
# TOKENS
~~~zig
KwPackage,Ident,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwObjective,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,OpenCurly,
KwIn,KwPart,Colon,Ident,Semicolon,
KwReturn,Colon,Ident,Semicolon,
CloseCurly,
KwReturn,KwPart,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def 'TradeStudyTest'
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'TradeStudies::*')
    (part_def 'Engine')
    (part_usage 'engine1' : 'Engine')
    (part_usage 'engine2' : 'Engine')
    (sysml_decl 'engineTradeStudy' : 'TradeStudy'
      (sysml_decl : 'Engine' multiplicity value)
      (objective_member)
      (calc_usage :>> 'evaluationFunction'
        (part_usage in : 'Engine')
        (return_member))
      (return_member))))
~~~
# EXPECTED
~~~
semantic.unresolved_name 'TradeStudy'
semantic.unresolved_name 'MaximizeObjective'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'Real'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'TradeStudy'
semantic.unresolved_name 'MaximizeObjective'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'Real'
~~~
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "5ecbfd207914be179a6fe96fef80e4b2334d95310dc5511c78187acbacb45e0e") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "TradeStudyTest"))) (kind "package") (name "TradeStudyTest") (declared-name "TradeStudyTest") (range (start (line 0) (character 0)) (end (line 0) (character 393))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 4) (character 1)) (end (line 4) (character 17))) (parent (node (document "d0") (qualified-name "TradeStudyTest"))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (kind "part") (name "engine1") (declared-name "engine1") (range (start (line 5) (character 1)) (end (line 5) (character 22))) (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 5) (character 15)) (end (line 5) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (kind "part") (name "engine2") (declared-name "engine2") (range (start (line 6) (character 1)) (end (line 6) (character 22))) (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 6) (character 15)) (end (line 6) (character 21)))))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind "analysis") (name "engineTradeStudy") (declared-name "engineTradeStudy") (range (start (line 8) (character 1)) (end (line 8) (character 227))) (parent (node (document "d0") (qualified-name "TradeStudyTest"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy") (range none)))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (kind "subject") (name "") (range (start (line 9) (character 2)) (end (line 9) (character 46))) (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (kind "analysis result") (name "") (range (start (line 17) (character 2)) (end (line 17) (character 23))) (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (kind "part") (name "") (range (start (line 13) (character 3)) (end (line 13) (character 20))) (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 13) (character 13)) (end (line 13) (character 19)))))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (range (start (line 12) (character 2)) (end (line 12) (character 74))) (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (kind "return parameter") (name "") (range (start (line 14) (character 3)) (end (line 14) (character 17))) (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 10) (character 2)) (end (line 10) (character 32))) (parent (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (authored (relationships (typing (reference "MaximizeObjective") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 5) (character 15)) (end (line 5) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 6) (character 15)) (end (line 6) (character 21))) (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 13) (character 13)) (end (line 13) (character 19))) (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "TradeStudyTest::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximizeObjective") (range none) (outcome (status unresolved)))
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
