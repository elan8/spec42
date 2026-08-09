# META
~~~ini
description=SysML Training 33 (Analysis): Trade Study Analysis Example
type=file
~~~
# SOURCE
~~~sysml
package 'Trade Study Analysis Example' {
	private import ScalarValues::Real;
	private import TradeStudies::*;
	
	part def Engine;
	part engine4cyl : Engine;
	part engine6cyl : Engine;
	
	calc def PowerRollup { in engine : Engine; return : ISQ::PowerValue; }
	calc def MassRollup { in engine : Engine; return : ISQ::MassValue; }
	calc def EfficiencyRollup { in engine : Engine; return : Real; }
	calc def CostRollup { in engine : Engine; return : Real; }
	
	calc def EngineEvaluation { 
		in power : ISQ::PowerValue;
		in mass : ISQ::MassValue;
		in efficiency : Real;
		in cost : Real;
		return evaluation : Real;
		// Compute evaluation...
	}
		
	analysis engineTradeStudy : TradeStudy {
		subject : Engine = (engine4cyl, engine6cyl);
		objective : MaximizeObjective;

		calc :>> evaluationFunction {
			in part anEngine :>> alternative : Engine;
			
			calc powerRollup: PowerRollup { in engine = anEngine; return power; }
			calc massRollup: MassRollup { in engine = anEngine; return mass; }
			calc efficiencyRollup: EfficiencyRollup { in engine = anEngine; return efficiency; }
			calc costRollup: CostRollup { in engine = anEngine; return cost; }
			
			return :>> result : Real = EngineEvaluation(
				powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost
			);
		}
		
		return part :>> selectedAlternative : Engine;
	}
	
}
~~~
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwPart,Ident,Colon,Ident,Semicolon,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwReturn,Colon,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwReturn,Colon,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,KwIn,Ident,Colon,Ident,Semicolon,KwReturn,Colon,Ident,Semicolon,CloseCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
LineComment,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Colon,Ident,Eq,OpenParen,Ident,Comma,Ident,CloseParen,Semicolon,
KwObjective,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,OpenCurly,
KwIn,KwPart,Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwCalc,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,Semicolon,CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,Semicolon,CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,Semicolon,CloseCurly,
KwCalc,Ident,Colon,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,Semicolon,CloseCurly,
KwReturn,ColonGtGt,Ident,Colon,Ident,Eq,Ident,OpenParen,
Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,
CloseParen,Semicolon,
CloseCurly,
KwReturn,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''Trade Study Analysis Example''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'TradeStudies::*')
    (part_def 'Engine')
    (part_usage 'engine4cyl' : 'Engine')
    (part_usage 'engine6cyl' : 'Engine')
    (calc_def 'PowerRollup'
      (default_ref_usage in 'engine' : 'Engine')
      (return_member))
    (calc_def 'MassRollup'
      (default_ref_usage in 'engine' : 'Engine')
      (return_member))
    (calc_def 'EfficiencyRollup'
      (default_ref_usage in 'engine' : 'Engine')
      (return_member))
    (calc_def 'CostRollup'
      (default_ref_usage in 'engine' : 'Engine')
      (return_member))
    (calc_def 'EngineEvaluation'
      (default_ref_usage in 'power' : 'ISQ::PowerValue')
      (default_ref_usage in 'mass' : 'ISQ::MassValue')
      (default_ref_usage in 'efficiency' : 'Real')
      (default_ref_usage in 'cost' : 'Real')
      (return_member)
      (line_comment))
    (sysml_decl 'engineTradeStudy' : 'TradeStudy'
      (sysml_decl : 'Engine' value)
      (objective_member)
      (calc_usage :>> 'evaluationFunction'
        (part_usage in 'anEngine' :>> 'alternative' : 'Engine')
        (calc_usage 'powerRollup' : 'PowerRollup'
          (default_ref_usage in 'engine' value)
          (return_member))
        (calc_usage 'massRollup' : 'MassRollup'
          (default_ref_usage in 'engine' value)
          (return_member))
        (calc_usage 'efficiencyRollup' : 'EfficiencyRollup'
          (default_ref_usage in 'engine' value)
          (return_member))
        (calc_usage 'costRollup' : 'CostRollup'
          (default_ref_usage in 'engine' value)
          (return_member))
        (return_member))
      (return_member))))
~~~
# FORMAT
~~~sysml
package 'Trade Study Analysis Example' {
    private import ScalarValues::Real;
    private import TradeStudies::*;

    part def Engine;
    part engine4cyl : Engine;
    part engine6cyl : Engine;

    calc def PowerRollup { in engine : Engine; return : ISQ::PowerValue; }
    calc def MassRollup { in engine : Engine; return : ISQ::MassValue; }
    calc def EfficiencyRollup { in engine : Engine; return : Real; }
    calc def CostRollup { in engine : Engine; return : Real; }

    calc def EngineEvaluation {
        in power : ISQ::PowerValue;
        in mass : ISQ::MassValue;
        in efficiency : Real;
        in cost : Real;
        return evaluation : Real;
        // Compute evaluation...
    }

    analysis engineTradeStudy : TradeStudy {
        subject : Engine = (engine4cyl, engine6cyl);
        objective : MaximizeObjective;

        calc :>> evaluationFunction {
            in part anEngine :>> alternative : Engine;

            calc powerRollup: PowerRollup { in engine = anEngine; return power; }
            calc massRollup: MassRollup { in engine = anEngine; return mass; }
            calc efficiencyRollup: EfficiencyRollup { in engine = anEngine; return efficiency; }
            calc costRollup: CostRollup { in engine = anEngine; return cost; }

            return :>> result : Real = EngineEvaluation(
            powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost
            );
        }

        return part :>> selectedAlternative : Engine;
    }

}

~~~
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TradeStudy'
semantic.unresolved_name 'MaximizeObjective'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'alternative'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'selectedAlternative'
~~~
# PROBLEMS
~~~
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'TradeStudy'
semantic.unresolved_name 'MaximizeObjective'
semantic.unresolved_name 'evaluationFunction'
semantic.unresolved_name 'alternative'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'selectedAlternative'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (name "Trade Study Analysis Example") (declared-name "Trade Study Analysis Example")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::*"))) (name "*") (declared-name "*"))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (name "CostRollup") (declared-name "CostRollup")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (name "EfficiencyRollup") (declared-name "EfficiencyRollup")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
          )
        )
        (element (kind "part def") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (name "Engine") (declared-name "Engine") (declared))
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (name "EngineEvaluation") (declared-name "EngineEvaluation")
          (contains
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (name "cost") (declared-name "cost") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (name "efficiency") (declared-name "efficiency") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation")))))
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (name "evaluation") (declared-name "evaluation") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::mass"))) (name "mass") (declared-name "mass") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::power"))) (name "power") (declared-name "power") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (name "MassRollup") (declared-name "MassRollup")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup")))))
          )
        )
        (element (kind "calc def") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (name "PowerRollup") (declared-name "PowerRollup")
          (contains
            (element (kind "return parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
            (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "part") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (name "engine4cyl") (declared-name "engine4cyl") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "part") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (name "engine6cyl") (declared-name "engine6cyl") (declared (properties (composite true) (reference false) (ordered false))))
        (element (kind "analysis") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (name "engineTradeStudy") (declared-name "engineTradeStudy")
          (contains
            (element (kind "subject") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (name ""))
            (element (kind "calc") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (name "evaluationFunction") (declared-name "evaluationFunction")
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (name "anEngine") (declared-name "anEngine") (declared (properties (direction "in") (composite true) (reference false) (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                (element (kind "calc") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (name "costRollup") (declared-name "costRollup")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup")))))
                  )
                )
                (element (kind "calc") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (name "efficiencyRollup") (declared-name "efficiencyRollup")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
                  )
                )
                (element (kind "calc") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (name "massRollup") (declared-name "massRollup")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup")))))
                  )
                )
                (element (kind "calc") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (name "powerRollup") (declared-name "powerRollup")
                  (contains
                    (element (kind "in out parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (name "engine") (declared-name "engine") (effective (featuring-type (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
                  )
                )
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (name "result") (declared-name "result"))
              )
            )
            (element (kind "objective") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::objective"))) (name "objective") (declared-name "objective"))
            (element (kind "analysis result") (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (name "selectedAlternative") (declared-name "selectedAlternative"))
          )
        )
      )
    )
  )
  (relationships
    (subject (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))))
    (typing (status resolved) (from (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (to (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))))
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
  (document "sysml/training/33_trade_study_analysis_example.md"
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
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 8 44) (end 8 69))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 9 43) (end 9 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 10 49) (end 10 63))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 11 43) (end 11 57))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 14 2) (end 14 29))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 15 2) (end 15 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 16 2) (end 16 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 17 2) (end 17 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 18 2) (end 18 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 1) (end 22 706))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 24 2) (end 24 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 27 3) (end 27 45))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 34 3) (end 34 138))
      )
    )
  )
)
~~~
