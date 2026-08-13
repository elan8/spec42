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
  (document "memory://snapshot/33_trade_study_analysis_example.md"
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
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 8 24) (end 8 43))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 8 44) (end 8 69))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 9 23) (end 9 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 9 43) (end 9 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 10 29) (end 10 48))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 10 49) (end 10 63))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 23) (end 11 42))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 11 43) (end 11 57))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 14 2) (end 14 29))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 15 2) (end 15 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 16 2) (end 16 23))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 17 2) (end 17 17))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 18 2) (end 18 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 22 29) (end 22 39))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 23 2) (end 23 46))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 24 2) (end 24 32))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 26 11) (end 26 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 27 24) (end 27 35))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 29 35) (end 29 56))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 29 57) (end 29 70))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 33) (end 30 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 30 55) (end 30 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 45) (end 31 66))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 31 67) (end 31 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 33) (end 32 54))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 32 55) (end 32 67))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_calc_definition_member")
        (source "semantic")
        (range (start 34 3) (end 36 5))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 39 2) (end 39 47))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation false) (source-digest "blake3:274a63a85b793cf406d13f0fe83aaf7af1f4187ddc54484969c9785666688456") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TradeStudies") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::CostRollup"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::EngineEvaluation"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::MassRollup"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TradeStudy"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind calc) (ordinal 0))))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "evaluationFunction"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")) (redefinition (reference "alternative"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::costRollup"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CostRollup"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::efficiencyRollup"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "EfficiencyRollup"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::massRollup"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "MassRollup"))))
    (declaration (id (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::powerRollup"))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "PowerRollup"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TradeStudies")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind featureTyping) (ordinal 0))
      (authored-target "TradeStudy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "evaluationFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine")))))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (kind redefinition) (ordinal 0))
      (authored-target "alternative")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::costRollup"))) (kind featureTyping) (ordinal 0))
      (authored-target "CostRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::CostRollup")))))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::efficiencyRollup"))) (kind featureTyping) (ordinal 0))
      (authored-target "EfficiencyRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::massRollup"))) (kind featureTyping) (ordinal 0))
      (authored-target "MassRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::MassRollup")))))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::powerRollup"))) (kind featureTyping) (ordinal 0))
      (authored-target "PowerRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::costRollup"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::CostRollup"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::costRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::efficiencyRollup"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::EfficiencyRollup"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::efficiencyRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::massRollup"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::MassRollup"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::massRollup"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::powerRollup"))) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::PowerRollup"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::powerRollup"))) (kind featureTyping) (ordinal 0)))
  )
  (evaluation
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 5 19) (end 5 25)) (probe (position 5 19))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine4cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine")))))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 6 19) (end 6 25)) (probe (position 6 19))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engine6cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine")))))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 22 29) (end 22 39)) (probe (position 22 29))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy"))) (kind featureTyping) (ordinal 0) (authored-target "TradeStudy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 26 11) (end 26 29)) (probe (position 26 11))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "evaluationFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 27 38) (end 27 44)) (probe (position 27 38))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::Engine")))))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 27 24) (end 27 35)) (probe (position 27 24))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::anEngine"))) (kind redefinition) (ordinal 0) (authored-target "alternative")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 32 20) (end 32 30)) (probe (position 32 20))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::costRollup"))) (kind featureTyping) (ordinal 0) (authored-target "CostRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::CostRollup")))))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 31 26) (end 31 42)) (probe (position 31 26))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::efficiencyRollup"))) (kind featureTyping) (ordinal 0) (authored-target "EfficiencyRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::EfficiencyRollup")))))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 30 20) (end 30 30)) (probe (position 30 20))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::massRollup"))) (kind featureTyping) (ordinal 0) (authored-target "MassRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::MassRollup")))))
  )
  (query (document "memory://snapshot/33_trade_study_analysis_example.md") (range (start 29 21) (end 29 32)) (probe (position 29 21))
    (reference (id (source (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::engineTradeStudy::::powerRollup"))) (kind featureTyping) (ordinal 0) (authored-target "PowerRollup")
      (outcome (status resolved) (target (node (document "memory://snapshot/33_trade_study_analysis_example.md") (qualified-name "Trade Study Analysis Example::PowerRollup")))))
  )
)
~~~
