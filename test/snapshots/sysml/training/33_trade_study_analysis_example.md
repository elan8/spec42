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
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "33_trade_study_analysis_example.md"
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
        (range (start 8 44) (end 8 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 9 43) (end 9 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 14 2) (end 14 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 15 2) (end 15 27))
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
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 24) (end 27 35))
      )
    )
  )
)
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f2716e199f468443dea392071901d1c1c4076bd6b82a650afbb4e2cea0fc7211") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (kind "package") (name "Trade Study Analysis Example") (declared-name "Trade Study Analysis Example") (range (start (line 0) (character 0)) (end (line 0) (character 1357))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (kind "calc def") (name "CostRollup") (declared-name "CostRollup") (range (start (line 11) (character 1)) (end (line 11) (character 59))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (kind "return parameter") (name "") (range (start (line 11) (character 43)) (end (line 11) (character 57))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 11) (character 23)) (end (line 11) (character 42))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (kind "calc def") (name "EfficiencyRollup") (declared-name "EfficiencyRollup") (range (start (line 10) (character 1)) (end (line 10) (character 65))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (kind "return parameter") (name "") (range (start (line 10) (character 49)) (end (line 10) (character 63))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 10) (character 29)) (end (line 10) (character 48))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 4) (character 1)) (end (line 4) (character 17))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (kind "calc def") (name "EngineEvaluation") (declared-name "EngineEvaluation") (range (start (line 13) (character 1)) (end (line 13) (character 187))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (kind "in out parameter") (name "cost") (declared-name "cost") (range (start (line 17) (character 2)) (end (line 17) (character 17))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (kind "in out parameter") (name "efficiency") (declared-name "efficiency") (range (start (line 16) (character 2)) (end (line 16) (character 23))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (kind "return parameter") (name "evaluation") (declared-name "evaluation") (range (start (line 18) (character 2)) (end (line 18) (character 27))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (range (start (line 15) (character 2)) (end (line 15) (character 27))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 14) (character 2)) (end (line 14) (character 29))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (kind "calc def") (name "MassRollup") (declared-name "MassRollup") (range (start (line 9) (character 1)) (end (line 9) (character 69))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::"))) (kind "return parameter") (name "") (range (start (line 9) (character 43)) (end (line 9) (character 67))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (authored (relationships (typing (reference "ISQ::MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 9) (character 23)) (end (line 9) (character 42))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (kind "calc def") (name "PowerRollup") (declared-name "PowerRollup") (range (start (line 8) (character 1)) (end (line 8) (character 71))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::"))) (kind "return parameter") (name "") (range (start (line 8) (character 44)) (end (line 8) (character 69))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (authored (relationships (typing (reference "ISQ::PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 8) (character 24)) (end (line 8) (character 43))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind "part") (name "engine4cyl") (declared-name "engine4cyl") (range (start (line 5) (character 1)) (end (line 5) (character 26))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 5) (character 19)) (end (line 5) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind "part") (name "engine6cyl") (declared-name "engine6cyl") (range (start (line 6) (character 1)) (end (line 6) (character 26))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 6) (character 19)) (end (line 6) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind "analysis") (name "engineTradeStudy") (declared-name "engineTradeStudy") (range (start (line 22) (character 1)) (end (line 22) (character 706))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (kind "subject") (name "") (range (start (line 23) (character 2)) (end (line 23) (character 46))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind "part") (name "anEngine") (declared-name "anEngine") (range (start (line 27) (character 3)) (end (line 27) (character 45))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 27) (character 38)) (end (line 27) (character 44)))) (redefinition (reference "alternative") (range (start (line 27) (character 24)) (end (line 27) (character 35)))))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (range (start (line 26) (character 2)) (end (line 26) (character 529))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (kind "calc") (name "costRollup") (declared-name "costRollup") (range (start (line 32) (character 3)) (end (line 32) (character 69))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "CostRollup") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 32) (character 33)) (end (line 32) (character 54))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind "calc") (name "efficiencyRollup") (declared-name "efficiencyRollup") (range (start (line 31) (character 3)) (end (line 31) (character 87))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "EfficiencyRollup") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 31) (character 45)) (end (line 31) (character 66))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (kind "calc") (name "massRollup") (declared-name "massRollup") (range (start (line 30) (character 3)) (end (line 30) (character 69))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRollup") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 30) (character 33)) (end (line 30) (character 54))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (kind "calc") (name "powerRollup") (declared-name "powerRollup") (range (start (line 29) (character 3)) (end (line 29) (character 72))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerRollup") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 29) (character 35)) (end (line 29) (character 56))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (kind "return parameter") (name "result") (declared-name "result") (range (start (line 34) (character 3)) (end (line 34) (character 138))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 24) (character 2)) (end (line 24) (character 32))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (authored (relationships (typing (reference "MaximizeObjective") (range none)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (kind "analysis result") (name "selectedAlternative") (declared-name "selectedAlternative") (range (start (line 39) (character 2)) (end (line 39) (character 47))) (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (authored (relationships (typing (reference "Engine") (range none)))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 5) (character 19)) (end (line 5) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 6) (character 19)) (end (line 6) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 27) (character 38)) (end (line 27) (character 44))) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind redefinition) (ordinal 0)) (authored-target "alternative") (range (start (line 27) (character 24)) (end (line 27) (character 35))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "CostRollup") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "EfficiencyRollup") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRollup") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerRollup") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximizeObjective") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 5 19) (end 5 25)) (probe (position 5 19))
      (reference
        (source (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 5 19) (end 5 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Trade Study Analysis Example::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 6 19) (end 6 25)) (probe (position 6 19))
      (reference
        (source (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 6 19) (end 6 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Trade Study Analysis Example::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 27 38) (end 27 44)) (probe (position 27 38))
      (reference
        (source (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 27 38) (end 27 44))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "Trade Study Analysis Example::Engine") (range (start 4 1) (end 4 17)))
        )
      )
    )
    (query (range (start 27 24) (end 27 35)) (probe (position 27 24))
      (reference
        (source (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))
        (kind redefinition) (ordinal 0) (authored-target "alternative")
        (range (start 27 24) (end 27 35))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "Trade Study Analysis Example::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "Trade Study Analysis Example::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
