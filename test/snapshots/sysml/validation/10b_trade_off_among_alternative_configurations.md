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
  (document "10b_trade_off_among_alternative_configurations.md"
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
        (range (start 66 3) (end 66 30))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 67 3) (end 67 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 74 2) (end 74 702))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 76 3) (end 76 33))
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
        (range (start 81 45) (end 81 70))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 82 44) (end 82 67))
      )
    )
  )
)
~~~
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6997af633d238122ea4a3258b938a8d85b009f947cc786634ebfb56a81603f43") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (kind "package") (name "10b-Trade-off Among Alternative Configurations") (declared-name "10b-Trade-off Among Alternative Configurations"))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import2"))) (kind "import") (name "*") (declared-name "*") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))) (kind "package") (name "Analysis") (declared-name "Analysis") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (kind "calc def") (name "EngineEvaluation") (declared-name "EngineEvaluation") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::_documentation"))) (kind "documentation") (name "") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind "in out parameter") (name "cost") (declared-name "cost") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind "in out parameter") (name "efficiency") (declared-name "efficiency") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind "return parameter") (name "evaluation") (declared-name "evaluation") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::MassValue")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind "in out parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::PowerValue")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind "analysis") (name "engineTradeStudy") (declared-name "engineTradeStudy") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (kind "subject") (name "") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind "part") (name "anEngine") (declared-name "anEngine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")) (redefinition (reference "alternative")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (kind "calc") (name "costRollup") (declared-name "costRollup") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (kind "return parameter") (name "cost") (declared-name "cost") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind "calc") (name "efficiencyRollup") (declared-name "efficiencyRollup") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (kind "return parameter") (name "efficiency") (declared-name "efficiency") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (kind "calc") (name "massRollup") (declared-name "massRollup") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::mass"))) (kind "return parameter") (name "mass") (declared-name "mass") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (authored (relationships (typing (reference "ISQ::mass")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (kind "calc") (name "powerRollup") (declared-name "powerRollup") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (authored (relationships (typing (reference "")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::power"))) (kind "return parameter") (name "power") (declared-name "power") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (authored (relationships (typing (reference "ISQ::power")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (kind "return parameter") (name "result") (declared-name "result") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (authored (relationships (typing (reference "Real")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::objective"))) (kind "objective") (name "objective") (declared-name "objective") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (authored (relationships (typing (reference "MaximizeObjective")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (kind "analysis result") (name "selectedAlternative") (declared-name "selectedAlternative") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (authored (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind "part def") (name "4CylCrankShaft") (declared-name "4CylCrankShaft") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CrankShaft")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind "part def") (name "6CylCrankShaft") (declared-name "6CylCrankShaft") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CrankShaft")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (kind "part def") (name "ConnectingRod") (declared-name "ConnectingRod") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (kind "part def") (name "CrankShaft") (declared-name "CrankShaft") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (kind "part def") (name "Piston") (declared-name "Piston") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (kind "import") (name "Real") (declared-name "Real") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind "part") (name "cs") (declared-name "cs") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "CrankShaft")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind "part") (name "p") (declared-name "p") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (authored (membership (kind Feature)) (relationships (typing (reference "Piston")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind "part") (name "rod") (declared-name "rod") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectingRod")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind "part") (name "engineChoice") (declared-name "engineChoice") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (kind "part") (name "cs") (declared-name "cs") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (authored (membership (kind Feature)) (relationships (typing (reference "4CylCrankShaft")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (kind "part") (name "cs") (declared-name "cs") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (authored (membership (kind Feature)) (relationships (typing (reference "6CylCrankShaft")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (kind "part") (name "cyl") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle")))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engineChoice")))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (outcome (status unresolved)) (import (origin import) (shape namespace) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages")))) (import (origin import) (shape namespace) (recursive false) (conformance valid)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind redefinition) (ordinal 0)) (authored-target "alternative") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::power") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximizeObjective") (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind specialization) (ordinal 0)) (authored-target "CrankShaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind specialization) (ordinal 0)) (authored-target "CrankShaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (outcome (status unresolved)) (import (origin import) (shape membership) (recursive false) (conformance not-checked-unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind featureTyping) (ordinal 0)) (authored-target "CrankShaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind featureTyping) (ordinal 0)) (authored-target "Piston") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectingRod") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (kind featureTyping) (ordinal 0)) (authored-target "4CylCrankShaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (kind featureTyping) (ordinal 0)) (authored-target "6CylCrankShaft") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind subsetting) (ordinal 0)) (authored-target "engineChoice") (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice")))))
  )
  (relationships
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subject) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance (derived CaseSubjectFromTypedSubject)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind specialization) (ordinal 0)))
    (relationship (kind specializes) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind specialization) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind subsetting) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind redefinition) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (kind redefinition) (ordinal 0)))
    (relationship (kind typing) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind featureTyping) (ordinal 0)))
    (relationship (kind subsetting) (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (provenance authored) (authored-reference (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind subsetting) (ordinal 0)))
  )
  (evaluation
    (node (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
    (node (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine")) (expression (status "incomplete") (error "expression is incomplete")))
  )
)
~~~
# NAVIGATION
~~~sexpr
(navigation
  (document "d0"
    (query (range (start 41 13) (end 41 16)) (probe (position 41 13))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 41 13) (end 41 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl") (range (start 41 4) (end 41 20)))
        )
      )
    )
    (query (range (start 46 13) (end 46 16)) (probe (position 46 13))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))
        (kind redefinition) (ordinal 0) (authored-target "cyl")
        (range (start 46 13) (end 46 16))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl") (range (start 46 4) (end 46 20)))
        )
      )
    )
    (query (range (start 4 16) (end 4 22)) (probe (position 4 16))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import2"))
        (kind namespaceImport) (ordinal 0) (authored-target "Usages::*")
        (range (start 4 16) (end 4 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages") (range (start 28 1) (end 28 708)))
        )
      )
    )
    (query (range (start 30 16) (end 30 22)) (probe (position 30 16))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 30 16) (end 30 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine") (range (start 10 2) (end 10 138)))
        )
      )
    )
    (query (range (start 32 16) (end 32 22)) (probe (position 32 16))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))
        (kind featureTyping) (ordinal 0) (authored-target "Piston")
        (range (start 32 16) (end 32 22))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston") (range (start 18 2) (end 18 18)))
        )
      )
    )
    (query (range (start 39 33) (end 39 39)) (probe (position 39 33))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))
        (kind subsetting) (ordinal 0) (authored-target "engine")
        (range (start 39 33) (end 39 39))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine") (range (start 30 2) (end 30 147)))
        )
      )
    )
    (query (range (start 79 39) (end 79 45)) (probe (position 79 39))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))
        (kind featureTyping) (ordinal 0) (authored-target "Engine")
        (range (start 79 39) (end 79 45))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine") (range (start 10 2) (end 10 138)))
        )
      )
    )
    (query (range (start 51 17) (end 51 24)) (probe (position 51 17))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))
        (kind featureTyping) (ordinal 0) (authored-target "Vehicle")
        (range (start 51 17) (end 51 24))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle") (range (start 8 2) (end 8 19)))
        )
      )
    )
    (query (range (start 31 17) (end 31 25)) (probe (position 31 17))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))
        (kind featureTyping) (ordinal 0) (authored-target "Cylinder")
        (range (start 31 17) (end 31 25))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder") (range (start 19 2) (end 19 20)))
        )
      )
    )
    (query (range (start 23 31) (end 23 41)) (probe (position 23 31))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))
        (kind specialization) (ordinal 0) (authored-target "CrankShaft")
        (range (start 23 31) (end 23 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft") (range (start 21 2) (end 21 22)))
        )
      )
    )
    (query (range (start 24 31) (end 24 41)) (probe (position 24 31))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))
        (kind specialization) (ordinal 0) (authored-target "CrankShaft")
        (range (start 24 31) (end 24 41))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft") (range (start 21 2) (end 21 22)))
        )
      )
    )
    (query (range (start 36 13) (end 36 23)) (probe (position 36 13))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))
        (kind featureTyping) (ordinal 0) (authored-target "CrankShaft")
        (range (start 36 13) (end 36 23))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft") (range (start 21 2) (end 21 22)))
        )
      )
    )
    (query (range (start 3 16) (end 3 27)) (probe (position 3 16))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import"))
        (kind namespaceImport) (ordinal 0) (authored-target "Definitions::*")
        (range (start 3 16) (end 3 27))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions") (range (start 6 1) (end 6 374)))
        )
      )
    )
    (query (range (start 79 25) (end 79 36)) (probe (position 79 25))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))
        (kind redefinition) (ordinal 0) (authored-target "alternative")
        (range (start 79 25) (end 79 36))
        (outcome (status unresolved))
      )
    )
    (query (range (start 2 16) (end 2 28)) (probe (position 2 16))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*"))
        (kind namespaceImport) (ordinal 0) (authored-target "TradeStudies::*")
        (range (start 2 16) (end 2 28))
        (outcome (status unresolved))
      )
    )
    (query (range (start 52 21) (end 52 33)) (probe (position 52 21))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))
        (kind subsetting) (ordinal 0) (authored-target "engineChoice")
        (range (start 52 21) (end 52 33))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice") (range (start 39 2) (end 39 235)))
        )
      )
    )
    (query (range (start 33 18) (end 33 31)) (probe (position 33 18))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))
        (kind featureTyping) (ordinal 0) (authored-target "ConnectingRod")
        (range (start 33 18) (end 33 31))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod") (range (start 20 2) (end 20 25)))
        )
      )
    )
    (query (range (start 42 18) (end 42 34)) (probe (position 42 18))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))
        (kind featureTyping) (ordinal 0) (authored-target "4CylCrankShaft")
        (range (start 42 18) (end 42 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft") (range (start 23 2) (end 23 42)))
        )
      )
    )
    (query (range (start 47 18) (end 47 34)) (probe (position 47 18))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))
        (kind featureTyping) (ordinal 0) (authored-target "6CylCrankShaft")
        (range (start 47 18) (end 47 34))
        (outcome (status resolved)
          (target (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft") (range (start 24 2) (end 24 42)))
        )
      )
    )
    (query (range (start 1 16) (end 1 34)) (probe (position 1 16))
      (reference
        (source (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))
        (kind membershipImport) (ordinal 0) (authored-target "ScalarValues::Real")
        (range (start 1 16) (end 1 34))
        (outcome (status unresolved))
      )
    )
  )
)
~~~
