# META
~~~ini
description=SysML Validation (10-Analysis and Trades): 10b-Trade-off Among Alternative Configurations
type=file
~~~
# SOURCE
~~~sysml
package '10b-Trade-off Among Alternative Configurations' {
	private import ScalarValues::Real;
	private import TradeStudies::*;
	private import Definitions::*;
	private import Usages::*;
	
	package Definitions {
		
		part def Vehicle;
		
		part def Engine {
			power : ISQ::PowerValue;
			mass : ISQ::MassValue;
			efficiency : Real;
			reliability : Real;
			cost : Real;
		}
		
		part def Piston;
		part def Cylinder;
		part def ConnectingRod;
		part def CrankShaft;
		
		part def '4CylCrankShaft' :> CrankShaft;
		part def '6CylCrankShaft' :> CrankShaft;
		
	}
	
	package Usages {
		
		part engine : Engine {
			part cyl[*] : Cylinder {
				part p[1] : Piston;
				part rod[1] : ConnectingRod;
			}
			
			part cs : CrankShaft;
		}
		
		variation part engineChoice :> engine {
			variant part '4cylEngine' {
				part :>> cyl[4];
				part :>> cs : '4CylCrankShaft';
			}
			
			variant part '6cylEngine' {
				part :>> cyl[6];
				part :>> cs : '6CylCrankShaft';
			}
		}
		
		part vehicle : Vehicle {
			part engine[1] :> engineChoice = engineChoice::'6cylEngine' {
				assert constraint engineSelectionRational { 
					doc /* Selected the best engine based on the 'engineTradeStudy'. */
					engine == Analysis::engineTradeStudy.selectedAlternative
				}
			}
			
		}
	}
	
	package Analysis {

		calc def EngineEvaluation {
			doc /* Evaluation function with criteria power, mass, efficency and cost. */
			in power : ISQ::PowerValue;
			in mass : ISQ::MassValue; 
			in efficiency : Real; 
			in cost : Real;
			return evaluation : Real;
			// Compute evaluation...
		}
			
		analysis engineTradeStudy : TradeStudy {
			subject : Engine[1..*] = all engineChoice;
			objective : MaximizeObjective;

			calc :>> evaluationFunction {
				in part anEngine :>> alternative : Engine;
				
				calc powerRollup { in engine = anEngine; return power:>ISQ::power; }
				calc massRollup { in engine = anEngine; return mass:>ISQ::mass; }
				calc efficiencyRollup { in engine = anEngine; return efficiency: Real; }
				calc costRollup { in engine = anEngine; return cost: Real; }
				
				return :>> result : Real = EngineEvaluation(
					powerRollup.power, massRollup.mass, efficiencyRollup.efficiency, costRollup.cost
				);
			}
			
			return part :>> selectedAlternative : Engine;
		}
        
	}
	
}
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md"
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
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 11 3) (end 11 27))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 12 3) (end 12 25))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 13 3) (end 13 21))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 14 3) (end 14 22))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_definition_member")
        (source "semantic")
        (range (start 15 3) (end 15 15))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 40 3) (end 43 4))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_part_usage_member")
        (source "semantic")
        (range (start 45 3) (end 48 4))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 55 15) (end 55 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 66 14) (end 66 29))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 13) (end 67 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 19) (end 68 23))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 13) (end 69 17))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 23) (end 70 27))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 30) (end 74 40))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 76 3) (end 76 33))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 78 12) (end 78 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 79 25) (end 79 36))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 81 59) (end 81 69))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 57) (end 82 66))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 69) (end 83 73))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 57) (end 84 61))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 24) (end 86 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 5) (end 87 22))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 24) (end 87 39))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 41) (end 87 68))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_reference")
        (source "semantic")
        (range (start 87 70) (end 87 85))
      )
      (diagnostic
        (severity warning)
        (code "unsupported_analysis_case_definition_member")
        (source "semantic")
        (range (start 91 3) (end 91 48))
      )
    )
  )
)
~~~
# SMG
~~~sexpr
(semantic-model
  (publication (phase resolved) (completeness unsupported-syntax) (has-evaluation true) (source-digest "blake3:7e89183f84ffb56dd10ee206e96505d2bd0fc0351bbfd85f4a1f35f1b44c7ac4") (contract-version "parser-owned-resolution-v1"))
  (declarations
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 0))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (membershipImport (reference "ScalarValues::Real") (import (shape membership) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 1))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "TradeStudies") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 2))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Definitions") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 3))))) (kind import) (membership (kind import) (visibility private)) (authored (membership (kind import) (visibility private)) (relationships (namespaceImport (reference "Usages") (import (shape namespace) (recursive false)))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (kind calc-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real") (direction in))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::MassValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::PowerValue") (direction in))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind analysis) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "TradeStudy"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind subject) (ordinal 0))))) (kind subject) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind calc) (ordinal 0))))) (kind calc) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (redefinition (reference "evaluationFunction"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine")) (redefinition (reference "alternative"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::cost"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::engine"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "anEngine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::efficiency"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::engine"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "anEngine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::engine"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "anEngine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::mass"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::mass"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup"))) (kind calc) (membership (kind feature) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::engine"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "anEngine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::power"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ISQ::power"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind parameter) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Real")) (memberAccessOperand (reference "powerRollup::power")) (memberAccessOperand (reference "massRollup::mass")) (memberAccessOperand (reference "efficiencyRollup::efficiency")) (memberAccessOperand (reference "costRollup::cost")) (invocationCallee (reference "EngineEvaluation"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CrankShaft"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind part-def) (membership (kind owning) (visibility default)) (authored (membership (kind owning) (visibility default)) (relationships (specialization (reference "CrankShaft"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (kind part-def) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (kind package) (membership (kind owning) (visibility default)))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Engine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "CrankShaft"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Cylinder"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Piston"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "ConnectingRod"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engine"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (featureTyping (reference "Vehicle"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind part) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (subsetting (reference "engineChoice"))))
    (declaration (id (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (kind constraint) (membership (kind feature) (visibility default)) (authored (membership (kind feature) (visibility default)) (relationships (expressionOperand (reference "engine")) (memberAccessOperand (reference "Analysis::engineTradeStudy::selectedAlternative"))))
  )
  (references
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0))
      (authored-target "TradeStudies")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0))
      (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0))
      (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::MassValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::PowerValue")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind featureTyping) (ordinal 0))
      (authored-target "TradeStudy")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind subject) (ordinal 0))))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0))
      (authored-target "evaluationFunction")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (kind redefinition) (ordinal 0))
      (authored-target "alternative")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::cost"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::engine"))) (kind expressionOperand) (ordinal 0))
      (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::efficiency"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::engine"))) (kind expressionOperand) (ordinal 0))
      (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::engine"))) (kind expressionOperand) (ordinal 0))
      (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::mass"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::engine"))) (kind expressionOperand) (ordinal 0))
      (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::power"))) (kind featureTyping) (ordinal 0))
      (authored-target "ISQ::power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind featureTyping) (ordinal 0))
      (authored-target "Real")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "powerRollup::power")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 1))
      (authored-target "massRollup::mass")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 2))
      (authored-target "efficiencyRollup::efficiency")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 3))
      (authored-target "costRollup::cost")
      (outcome (status unresolved)))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind invocationCallee) (ordinal 0))
      (authored-target "EngineEvaluation")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind specialization) (ordinal 0))
      (authored-target "CrankShaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind specialization) (ordinal 0))
      (authored-target "CrankShaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind featureTyping) (ordinal 0))
      (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind featureTyping) (ordinal 0))
      (authored-target "CrankShaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind featureTyping) (ordinal 0))
      (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind featureTyping) (ordinal 0))
      (authored-target "Piston")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind featureTyping) (ordinal 0))
      (authored-target "ConnectingRod")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind subsetting) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind featureTyping) (ordinal 0))
      (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind subsetting) (ordinal 0))
      (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (kind expressionOperand) (ordinal 0))
      (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine")))))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (kind memberAccessOperand) (ordinal 0))
      (authored-target "Analysis::engineTradeStudy::selectedAlternative")
      (outcome (status unresolved)))
  )
  (relationships
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind subject) (ordinal 0))))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind subject) (ordinal 0))))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::engine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::engine"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::engine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::engine"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::engine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::engine"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::engine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::engine"))) (kind expressionOperand) (ordinal 0)))
    (relationship (kind invocationCallee) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind invocationCallee) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind specialization) (ordinal 0)))
    (relationship (kind specialization) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind subsetting) (ordinal 0)))
    (relationship (kind expressionOperand) (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (provenance authored) (authored-reference (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (kind expressionOperand) (ordinal 0)))
  )
  (evaluation
    (evaluated (declaration (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::engine"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::engine"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::engine"))) (value (kind non-constant)))
    (evaluated (declaration (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::engine"))) (value (kind non-constant)))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 2 16) (end 2 31)) (probe (position 2 16))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 1))))) (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 3 16) (end 3 30)) (probe (position 3 16))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 2))))) (kind namespaceImport) (ordinal 0) (authored-target "Definitions")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 4 16) (end 4 25)) (probe (position 4 16))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 3))))) (kind namespaceImport) (ordinal 0) (authored-target "Usages")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 1 16) (end 1 34)) (probe (position 1 16))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind import) (ordinal 0))))) (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 69 13) (end 69 17)) (probe (position 69 13))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 68 19) (end 68 23)) (probe (position 68 19))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 70 23) (end 70 27)) (probe (position 70 23))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 67 13) (end 67 27)) (probe (position 67 13))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::MassValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 66 14) (end 66 29)) (probe (position 66 14))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::PowerValue")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 74 30) (end 74 40)) (probe (position 74 30))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind featureTyping) (ordinal 0) (authored-target "TradeStudy")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 75 13) (end 75 19)) (probe (position 75 13))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind subject) (ordinal 0))))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 78 12) (end 78 30)) (probe (position 78 12))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (anonymous (kind calc) (ordinal 0))))) (kind redefinition) (ordinal 0) (authored-target "evaluationFunction")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 79 39) (end 79 45)) (probe (position 79 39))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 79 25) (end 79 36)) (probe (position 79 25))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine"))) (kind redefinition) (ordinal 0) (authored-target "alternative")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 84 57) (end 84 61)) (probe (position 84 57))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::cost"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 84 34) (end 84 42)) (probe (position 84 34))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::costRollup::engine"))) (kind expressionOperand) (ordinal 0) (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 83 69) (end 83 73)) (probe (position 83 69))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::efficiency"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 83 40) (end 83 48)) (probe (position 83 40))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::efficiencyRollup::engine"))) (kind expressionOperand) (ordinal 0) (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 82 34) (end 82 42)) (probe (position 82 34))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::engine"))) (kind expressionOperand) (ordinal 0) (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 82 57) (end 82 66)) (probe (position 82 57))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::massRollup::mass"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 81 35) (end 81 43)) (probe (position 81 35))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::engine"))) (kind expressionOperand) (ordinal 0) (authored-target "anEngine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::anEngine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 81 59) (end 81 69)) (probe (position 81 59))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::powerRollup::power"))) (kind featureTyping) (ordinal 0) (authored-target "ISQ::power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 86 24) (end 86 28)) (probe (position 86 24))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind featureTyping) (ordinal 0) (authored-target "Real")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 87 5) (end 87 22)) (probe (position 87 5))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 0) (authored-target "powerRollup::power")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 87 24) (end 87 39)) (probe (position 87 24))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 1) (authored-target "massRollup::mass")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 87 41) (end 87 68)) (probe (position 87 41))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 2) (authored-target "efficiencyRollup::efficiency")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 87 70) (end 87 85)) (probe (position 87 70))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind memberAccessOperand) (ordinal 3) (authored-target "costRollup::cost")
      (outcome (status unresolved)))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 86 31) (end 86 47)) (probe (position 86 31))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::::result"))) (kind invocationCallee) (ordinal 0) (authored-target "EngineEvaluation")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 23 31) (end 23 41)) (probe (position 23 31))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind specialization) (ordinal 0) (authored-target "CrankShaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 24 31) (end 24 41)) (probe (position 24 31))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind specialization) (ordinal 0) (authored-target "CrankShaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 30 16) (end 30 22)) (probe (position 30 16))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind featureTyping) (ordinal 0) (authored-target "Engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 36 13) (end 36 23)) (probe (position 36 13))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind featureTyping) (ordinal 0) (authored-target "CrankShaft")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 31 17) (end 31 25)) (probe (position 31 17))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 32 16) (end 32 22)) (probe (position 32 16))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind featureTyping) (ordinal 0) (authored-target "Piston")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 33 18) (end 33 31)) (probe (position 33 18))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind featureTyping) (ordinal 0) (authored-target "ConnectingRod")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 39 33) (end 39 39)) (probe (position 39 33))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind subsetting) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 51 17) (end 51 24)) (probe (position 51 17))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 52 21) (end 52 33)) (probe (position 52 21))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind subsetting) (ordinal 0) (authored-target "engineChoice")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 55 5) (end 55 11)) (probe (position 55 5))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (kind expressionOperand) (ordinal 0) (authored-target "engine")
      (outcome (status resolved) (target (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine")))))
  )
  (query (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (range (start 55 15) (end 55 61)) (probe (position 55 15))
    (reference (id (source (node (document "memory://snapshot/10b_trade_off_among_alternative_configurations.md") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine::engineSelectionRational"))) (kind memberAccessOperand) (ordinal 0) (authored-target "Analysis::engineTradeStudy::selectedAlternative")
      (outcome (status unresolved)))
  )
)
~~~
