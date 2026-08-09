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
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "TradeStudyTest"))) (name "TradeStudyTest") (declared-name "TradeStudyTest")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "TradeStudyTest::*"))) (name "*") (declared-name "*"))
        (element (kind "part def") (id (node (document "d0") (qualified-name "TradeStudyTest::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "import") (id (node (document "d0") (qualified-name "TradeStudyTest::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "part") (id (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (name "engine1") (declared-name "engine1") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (name "engine2") (declared-name "engine2") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "analysis") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (name "engineTradeStudy") (declared-name "engineTradeStudy")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (name ""))
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (name ""))
            (element (kind "calc") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction"))) (name "evaluationFunction") (declared-name "evaluationFunction")
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (name "") (declared (properties (direction "in") (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::evaluationFunction::"))) (name ""))
              )
            )
            (element (kind "objective") (id (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::objective"))) (name "objective") (declared-name "objective"))
          )
        )
      )
    )
  )
  (relationships
    (subject (status resolved) (from (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy"))) (to (node (document "d0") (qualified-name "TradeStudyTest::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudyTest::engine1"))) (to (node (document "d0") (qualified-name "TradeStudyTest::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudyTest::engine2"))) (to (node (document "d0") (qualified-name "TradeStudyTest::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::"))) (to (node (document "d0") (qualified-name "TradeStudyTest::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#analysis_result"))) (to (node (document "d0") (qualified-name "TradeStudyTest::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "TradeStudyTest::engineTradeStudy::#part"))) (to (node (document "d0") (qualified-name "TradeStudyTest::Engine"))))
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
  (document "sysml/examples/trade_study_test.md"
    (diagnostics
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 1 1) (end 1 35))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_import_target")
        (source "semantic")
        (range (start 2 1) (end 2 32))
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
