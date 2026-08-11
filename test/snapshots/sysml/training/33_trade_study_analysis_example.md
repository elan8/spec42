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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "f2716e199f468443dea392071901d1c1c4076bd6b82a650afbb4e2cea0fc7211") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (kind "package") (name "Trade Study Analysis Example") (declared-name "Trade Study Analysis Example"))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (kind "calc def") (name "CostRollup") (declared-name "CostRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (kind "calc def") (name "EfficiencyRollup") (declared-name "EfficiencyRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (kind "calc def") (name "EngineEvaluation") (declared-name "EngineEvaluation") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (kind "in out parameter") (name "cost") (declared-name "cost") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (kind "in out parameter") (name "efficiency") (declared-name "efficiency") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (kind "return parameter") (name "evaluation") (declared-name "evaluation") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::MassValue")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (kind "calc def") (name "MassRollup") (declared-name "MassRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (authored (relationships (typing (reference "ISQ::MassValue")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (kind "calc def") (name "PowerRollup") (declared-name "PowerRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::"))) (kind "return parameter") (name "") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (authored (relationships (typing (reference "ISQ::PowerValue")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind "part") (name "engine4cyl") (declared-name "engine4cyl") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind "part") (name "engine6cyl") (declared-name "engine6cyl") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind "analysis") (name "engineTradeStudy") (declared-name "engineTradeStudy") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind "part") (name "anEngine") (declared-name "anEngine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")) (redefinition (reference "alternative")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (kind "calc") (name "costRollup") (declared-name "costRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "CostRollup")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind "calc") (name "efficiencyRollup") (declared-name "efficiencyRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "EfficiencyRollup")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (kind "calc") (name "massRollup") (declared-name "massRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "MassRollup")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (kind "calc") (name "powerRollup") (declared-name "powerRollup") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "PowerRollup")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (kind "return parameter") (name "result") (declared-name "result") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (authored (relationships (typing (reference "MaximizeObjective")))))
    (element (id (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (kind "analysis result") (name "selectedAlternative") (declared-name "selectedAlternative") (parent (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (authored (relationships (typing (reference "Engine")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::EngineEvaluation::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::anEngine"))) (kind redefinition) (ordinal 0)) (authored-target "alternative") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "CostRollup") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::CostRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "EfficiencyRollup") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "MassRollup") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::MassRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup"))) (kind featureTyping) (ordinal 0)) (authored-target "PowerRollup") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::evaluationFunction::result"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximizeObjective") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "Trade Study Analysis Example::engineTradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "Trade Study Analysis Example::Engine")))))
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
