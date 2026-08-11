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
# FORMAT
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
# SMG
~~~
(semantic-model
  (publication (phase evaluated) (completeness complete) (has-evaluation true) (source-digest "6997af633d238122ea4a3258b938a8d85b009f947cc786634ebfb56a81603f43") (contract-version "canonical-resolution-v1"))
  (structure
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (kind "package") (name "10b-Trade-off Among Alternative Configurations") (declared-name "10b-Trade-off Among Alternative Configurations") (range (start (line 0) (character 0)) (end (line 0) (character 2297))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*"))) (kind "import") (name "*") (declared-name "*") (range (start (line 2) (character 1)) (end (line 2) (character 32))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "TradeStudies::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 2) (character 16)) (end (line 2) (character 28))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import"))) (kind "import") (name "*") (declared-name "*") (range (start (line 3) (character 1)) (end (line 3) (character 31))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Definitions::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 3) (character 16)) (end (line 3) (character 27))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import2"))) (kind "import") (name "*") (declared-name "*") (range (start (line 4) (character 1)) (end (line 4) (character 26))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "Usages::*") (origin Import) (shape Namespace) (recursive false)) (import-range (start (line 4) (character 16)) (end (line 4) (character 22))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))) (kind "package") (name "Analysis") (declared-name "Analysis") (range (start (line 62) (character 1)) (end (line 62) (character 1016))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (kind "calc def") (name "EngineEvaluation") (declared-name "EngineEvaluation") (range (start (line 64) (character 2)) (end (line 64) (character 276))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::_documentation"))) (kind "documentation") (name "") (range (start (line 64) (character 2)) (end (line 64) (character 276))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind "in out parameter") (name "cost") (declared-name "cost") (range (start (line 69) (character 3)) (end (line 69) (character 18))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind "in out parameter") (name "efficiency") (declared-name "efficiency") (range (start (line 68) (character 3)) (end (line 68) (character 24))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind "return parameter") (name "evaluation") (declared-name "evaluation") (range (start (line 70) (character 3)) (end (line 70) (character 28))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind "in out parameter") (name "mass") (declared-name "mass") (range (start (line 67) (character 3)) (end (line 67) (character 28))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::MassValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind "in out parameter") (name "power") (declared-name "power") (range (start (line 66) (character 3)) (end (line 66) (character 30))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (authored (relationships (typing (reference "ISQ::PowerValue") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind "analysis") (name "engineTradeStudy") (declared-name "engineTradeStudy") (range (start (line 74) (character 2)) (end (line 74) (character 702))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))) (authored (membership (kind Feature)) (relationships (typing (reference "TradeStudy") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (kind "subject") (name "") (range (start (line 75) (character 3)) (end (line 75) (character 45))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind "part") (name "anEngine") (declared-name "anEngine") (range (start (line 79) (character 4)) (end (line 79) (character 46))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 79) (character 39)) (end (line 79) (character 45)))) (redefinition (reference "alternative") (range (start (line 79) (character 25)) (end (line 79) (character 36)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (kind "calc") (name "evaluationFunction") (declared-name "evaluationFunction") (range (start (line 78) (character 3)) (end (line 78) (character 521))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (kind "calc") (name "costRollup") (declared-name "costRollup") (range (start (line 84) (character 4)) (end (line 84) (character 64))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (kind "return parameter") (name "cost") (declared-name "cost") (range (start (line 84) (character 44)) (end (line 84) (character 62))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 84) (character 22)) (end (line 84) (character 43))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (kind "calc") (name "efficiencyRollup") (declared-name "efficiencyRollup") (range (start (line 83) (character 4)) (end (line 83) (character 76))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (kind "return parameter") (name "efficiency") (declared-name "efficiency") (range (start (line 83) (character 50)) (end (line 83) (character 74))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 83) (character 28)) (end (line 83) (character 49))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (kind "calc") (name "massRollup") (declared-name "massRollup") (range (start (line 82) (character 4)) (end (line 82) (character 69))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 82) (character 22)) (end (line 82) (character 43))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::mass"))) (kind "return parameter") (name "mass") (declared-name "mass") (range (start (line 82) (character 44)) (end (line 82) (character 67))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (authored (relationships (typing (reference "ISQ::mass") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (kind "calc") (name "powerRollup") (declared-name "powerRollup") (range (start (line 81) (character 4)) (end (line 81) (character 72))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind "in out parameter") (name "engine") (declared-name "engine") (range (start (line 81) (character 23)) (end (line 81) (character 44))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (authored (relationships (typing (reference "") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::power"))) (kind "return parameter") (name "power") (declared-name "power") (range (start (line 81) (character 45)) (end (line 81) (character 70))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (authored (relationships (typing (reference "ISQ::power") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (kind "return parameter") (name "result") (declared-name "result") (range (start (line 86) (character 4)) (end (line 86) (character 141))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (authored (relationships (typing (reference "Real") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::objective"))) (kind "objective") (name "objective") (declared-name "objective") (range (start (line 76) (character 3)) (end (line 76) (character 33))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (authored (relationships (typing (reference "MaximizeObjective") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (kind "analysis result") (name "selectedAlternative") (declared-name "selectedAlternative") (range (start (line 91) (character 3)) (end (line 91) (character 48))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (authored (relationships (typing (reference "Engine") (range none)))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (kind "package") (name "Definitions") (declared-name "Definitions") (range (start (line 6) (character 1)) (end (line 6) (character 374))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind "part def") (name "4CylCrankShaft") (declared-name "4CylCrankShaft") (range (start (line 23) (character 2)) (end (line 23) (character 42))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CrankShaft") (range (start (line 23) (character 31)) (end (line 23) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind "part def") (name "6CylCrankShaft") (declared-name "6CylCrankShaft") (range (start (line 24) (character 2)) (end (line 24) (character 42))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (authored (membership (kind Owning)) (relationships (specializes (reference "CrankShaft") (range (start (line 24) (character 31)) (end (line 24) (character 41)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (kind "part def") (name "ConnectingRod") (declared-name "ConnectingRod") (range (start (line 20) (character 2)) (end (line 20) (character 25))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (kind "part def") (name "CrankShaft") (declared-name "CrankShaft") (range (start (line 21) (character 2)) (end (line 21) (character 22))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (kind "part def") (name "Cylinder") (declared-name "Cylinder") (range (start (line 19) (character 2)) (end (line 19) (character 20))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (kind "part def") (name "Engine") (declared-name "Engine") (range (start (line 10) (character 2)) (end (line 10) (character 138))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (kind "part def") (name "Piston") (declared-name "Piston") (range (start (line 18) (character 2)) (end (line 18) (character 18))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (kind "part def") (name "Vehicle") (declared-name "Vehicle") (range (start (line 8) (character 2)) (end (line 8) (character 19))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (kind "import") (name "Real") (declared-name "Real") (range (start (line 1) (character 1)) (end (line 1) (character 35))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (authored (membership (kind Import) (visibility "private") (import (reference "ScalarValues::Real") (origin Import) (shape Membership) (recursive false)) (import-range (start (line 1) (character 16)) (end (line 1) (character 34))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (kind "package") (name "Usages") (declared-name "Usages") (range (start (line 28) (character 1)) (end (line 28) (character 708))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 30) (character 2)) (end (line 30) (character 147))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Engine") (range (start (line 30) (character 16)) (end (line 30) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind "part") (name "cs") (declared-name "cs") (range (start (line 36) (character 3)) (end (line 36) (character 24))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "CrankShaft") (range (start (line 36) (character 13)) (end (line 36) (character 23)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind "part") (name "cyl") (declared-name "cyl") (range (start (line 31) (character 3)) (end (line 31) (character 89))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (authored (membership (kind Feature)) (relationships (typing (reference "Cylinder") (range (start (line 31) (character 17)) (end (line 31) (character 25)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind "part") (name "p") (declared-name "p") (range (start (line 32) (character 4)) (end (line 32) (character 23))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (authored (membership (kind Feature)) (relationships (typing (reference "Piston") (range (start (line 32) (character 16)) (end (line 32) (character 22)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind "part") (name "rod") (declared-name "rod") (range (start (line 33) (character 4)) (end (line 33) (character 32))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (authored (membership (kind Feature)) (relationships (typing (reference "ConnectingRod") (range (start (line 33) (character 18)) (end (line 33) (character 31)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind "part") (name "engineChoice") (declared-name "engineChoice") (range (start (line 39) (character 2)) (end (line 39) (character 235))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engine") (range (start (line 39) (character 33)) (end (line 39) (character 39)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (kind "part") (name "4cylEngine") (declared-name "4cylEngine") (range (start (line 40) (character 11)) (end (line 40) (character 92))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (kind "part") (name "cs") (declared-name "cs") (range (start (line 42) (character 4)) (end (line 42) (character 35))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (authored (membership (kind Feature)) (relationships (typing (reference "4CylCrankShaft") (range (start (line 42) (character 18)) (end (line 42) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (kind "part") (name "cyl") (range (start (line 41) (character 4)) (end (line 41) (character 20))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 41) (character 13)) (end (line 41) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (kind "part") (name "6cylEngine") (declared-name "6cylEngine") (range (start (line 45) (character 11)) (end (line 45) (character 92))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (kind "part") (name "cs") (declared-name "cs") (range (start (line 47) (character 4)) (end (line 47) (character 35))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (authored (membership (kind Feature)) (relationships (typing (reference "6CylCrankShaft") (range (start (line 47) (character 18)) (end (line 47) (character 34)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (kind "part") (name "cyl") (range (start (line 46) (character 4)) (end (line 46) (character 20))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (authored (membership (kind Feature)) (relationships (redefinition (reference "cyl") (range (start (line 46) (character 13)) (end (line 46) (character 16)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind "part") (name "vehicle") (declared-name "vehicle") (range (start (line 51) (character 2)) (end (line 51) (character 294))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (authored (membership (kind Feature)) (relationships (typing (reference "Vehicle") (range (start (line 51) (character 17)) (end (line 51) (character 24)))))))
    (element (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind "part") (name "engine") (declared-name "engine") (range (start (line 52) (character 3)) (end (line 52) (character 259))) (parent (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (authored (membership (kind Feature)) (relationships (subsetting (reference "engineChoice") (range (start (line 52) (character 21)) (end (line 52) (character 33)))))))
  )
  (references
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*"))) (kind namespaceImport) (ordinal 0)) (authored-target "TradeStudies::*") (range (start (line 2) (character 16)) (end (line 2) (character 28))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import"))) (kind namespaceImport) (ordinal 0)) (authored-target "Definitions::*") (range (start (line 3) (character 16)) (end (line 3) (character 27))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import2"))) (kind namespaceImport) (ordinal 0)) (authored-target "Usages::*") (range (start (line 4) (character 16)) (end (line 4) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::MassValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::PowerValue") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (kind featureTyping) (ordinal 0)) (authored-target "TradeStudy") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 79) (character 39)) (end (line 79) (character 45))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (kind redefinition) (ordinal 0)) (authored-target "alternative") (range (start (line 79) (character 25)) (end (line 79) (character 36))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::mass"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::mass") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::power"))) (kind featureTyping) (ordinal 0)) (authored-target "ISQ::power") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (kind featureTyping) (ordinal 0)) (authored-target "Real") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::objective"))) (kind featureTyping) (ordinal 0)) (authored-target "MaximizeObjective") (range none) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range none) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (kind specialization) (ordinal 0)) (authored-target "CrankShaft") (range (start (line 23) (character 31)) (end (line 23) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (kind specialization) (ordinal 0)) (authored-target "CrankShaft") (range (start (line 24) (character 31)) (end (line 24) (character 41))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (kind membershipImport) (ordinal 0)) (authored-target "ScalarValues::Real") (range (start (line 1) (character 16)) (end (line 1) (character 34))) (outcome (status unresolved)))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (kind featureTyping) (ordinal 0)) (authored-target "Engine") (range (start (line 30) (character 16)) (end (line 30) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (kind featureTyping) (ordinal 0)) (authored-target "CrankShaft") (range (start (line 36) (character 13)) (end (line 36) (character 23))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (kind featureTyping) (ordinal 0)) (authored-target "Cylinder") (range (start (line 31) (character 17)) (end (line 31) (character 25))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (kind featureTyping) (ordinal 0)) (authored-target "Piston") (range (start (line 32) (character 16)) (end (line 32) (character 22))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (kind featureTyping) (ordinal 0)) (authored-target "ConnectingRod") (range (start (line 33) (character 18)) (end (line 33) (character 31))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (kind subsetting) (ordinal 0)) (authored-target "engine") (range (start (line 39) (character 33)) (end (line 39) (character 39))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (kind featureTyping) (ordinal 0)) (authored-target "4CylCrankShaft") (range (start (line 42) (character 18)) (end (line 42) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 41) (character 13)) (end (line 41) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (kind featureTyping) (ordinal 0)) (authored-target "6CylCrankShaft") (range (start (line 47) (character 18)) (end (line 47) (character 34))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (kind redefinition) (ordinal 0)) (authored-target "cyl") (range (start (line 46) (character 13)) (end (line 46) (character 16))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (kind featureTyping) (ordinal 0)) (authored-target "Vehicle") (range (start (line 51) (character 17)) (end (line 51) (character 24))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle")))))
    (reference (id (source (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (kind subsetting) (ordinal 0)) (authored-target "engineChoice") (range (start (line 52) (character 21)) (end (line 52) (character 33))) (outcome (status resolved) (target (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice")))))
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
