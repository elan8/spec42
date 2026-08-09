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
# TOKENS
~~~zig
KwPackage,UnrestrictedName,OpenCurly,
KwPrivate,KwImport,Ident,ColonColon,Ident,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPrivate,KwImport,Ident,ColonColon,Star,Semicolon,
KwPackage,Ident,OpenCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,OpenCurly,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
Ident,Colon,Ident,ColonColon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
Ident,Colon,Ident,Semicolon,
CloseCurly,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
KwPart,KwDef,UnrestrictedName,ColonGt,Ident,Semicolon,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,OpenSquare,Star,CloseSquare,Colon,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,Colon,Ident,Semicolon,
CloseCurly,
KwPart,Ident,Colon,Ident,Semicolon,
CloseCurly,
KwVariation,KwPart,Ident,ColonGt,Ident,OpenCurly,
KwVariant,KwPart,UnrestrictedName,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,Colon,UnrestrictedName,Semicolon,
CloseCurly,
KwVariant,KwPart,UnrestrictedName,OpenCurly,
KwPart,ColonGtGt,Ident,OpenSquare,DecimalValue,CloseSquare,Semicolon,
KwPart,ColonGtGt,Ident,Colon,UnrestrictedName,Semicolon,
CloseCurly,
CloseCurly,
KwPart,Ident,Colon,Ident,OpenCurly,
KwPart,Ident,OpenSquare,DecimalValue,CloseSquare,ColonGt,Ident,Eq,Ident,ColonColon,UnrestrictedName,OpenCurly,
KwAssert,KwConstraint,Ident,OpenCurly,
KwDoc,RegularComment,
Ident,EqEq,Ident,ColonColon,Ident,Dot,Ident,
CloseCurly,
CloseCurly,
CloseCurly,
CloseCurly,
KwPackage,Ident,OpenCurly,
KwCalc,KwDef,Ident,OpenCurly,
KwDoc,RegularComment,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,ColonColon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwIn,Ident,Colon,Ident,Semicolon,
KwReturn,Ident,Colon,Ident,Semicolon,
LineComment,
CloseCurly,
KwAnalysis,Ident,Colon,Ident,OpenCurly,
KwSubject,Colon,Ident,OpenSquare,DecimalValue,DotDot,Star,CloseSquare,Eq,KwAll,Ident,Semicolon,
KwObjective,Colon,Ident,Semicolon,
KwCalc,ColonGtGt,Ident,OpenCurly,
KwIn,KwPart,Ident,ColonGtGt,Ident,Colon,Ident,Semicolon,
KwCalc,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwCalc,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,ColonGt,Ident,ColonColon,Ident,Semicolon,CloseCurly,
KwCalc,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwCalc,Ident,OpenCurly,KwIn,Ident,Eq,Ident,Semicolon,KwReturn,Ident,Colon,Ident,Semicolon,CloseCurly,
KwReturn,ColonGtGt,Ident,Colon,Ident,Eq,Ident,OpenParen,
Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,Comma,Ident,Dot,Ident,
CloseParen,Semicolon,
CloseCurly,
KwReturn,KwPart,ColonGtGt,Ident,Colon,Ident,Semicolon,
CloseCurly,
CloseCurly,
CloseCurly,EndOfFile,
~~~
# AST
~~~
(root
  (package_def ''10b-Trade-off Among Alternative Configurations''
    (import_decl private 'ScalarValues::Real')
    (import_decl private 'TradeStudies::*')
    (import_decl private 'Definitions::*')
    (import_decl private 'Usages::*')
    (package_def 'Definitions'
      (part_def 'Vehicle')
      (part_def 'Engine'
        (default_ref_usage 'power' : 'ISQ::PowerValue')
        (default_ref_usage 'mass' : 'ISQ::MassValue')
        (default_ref_usage 'efficiency' : 'Real')
        (default_ref_usage 'reliability' : 'Real')
        (default_ref_usage 'cost' : 'Real'))
      (part_def 'Piston')
      (part_def 'Cylinder')
      (part_def 'ConnectingRod')
      (part_def 'CrankShaft')
      (part_def ''4CylCrankShaft'' :> 'CrankShaft')
      (part_def ''6CylCrankShaft'' :> 'CrankShaft'))
    (package_def 'Usages'
      (part_usage 'engine' : 'Engine'
        (part_usage 'cyl' : 'Cylinder' multiplicity
          (part_usage 'p' : 'Piston' multiplicity)
          (part_usage 'rod' : 'ConnectingRod' multiplicity))
        (part_usage 'cs' : 'CrankShaft'))
      (part_usage variation 'engineChoice' :> 'engine'
        (variant_usage
          (part_usage ''4cylEngine''
            (part_usage :>> 'cyl' multiplicity)
            (part_usage :>> 'cs' : ''4CylCrankShaft'')))
        (variant_usage
          (part_usage ''6cylEngine''
            (part_usage :>> 'cyl' multiplicity)
            (part_usage :>> 'cs' : ''6CylCrankShaft''))))
      (part_usage 'vehicle' : 'Vehicle'
        (part_usage 'engine' :> 'engineChoice' multiplicity value
          (sysml_decl 'engineSelectionRational'
            (documentation)
            (result_expr_member)))))
    (package_def 'Analysis'
      (calc_def 'EngineEvaluation'
        (documentation)
        (default_ref_usage in 'power' : 'ISQ::PowerValue')
        (default_ref_usage in 'mass' : 'ISQ::MassValue')
        (default_ref_usage in 'efficiency' : 'Real')
        (default_ref_usage in 'cost' : 'Real')
        (return_member)
        (line_comment))
      (sysml_decl 'engineTradeStudy' : 'TradeStudy'
        (sysml_decl : 'Engine' multiplicity value)
        (objective_member)
        (calc_usage :>> 'evaluationFunction'
          (part_usage in 'anEngine' :>> 'alternative' : 'Engine')
          (calc_usage 'powerRollup'
            (default_ref_usage in 'engine' value)
            (return_member))
          (calc_usage 'massRollup'
            (default_ref_usage in 'engine' value)
            (return_member))
          (calc_usage 'efficiencyRollup'
            (default_ref_usage in 'engine' value)
            (return_member))
          (calc_usage 'costRollup'
            (default_ref_usage in 'engine' value)
            (return_member))
          (return_member))
        (return_member)))))
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
# EXPECTED
~~~
semantic.unresolved_name 'ISQ::PowerValue'
semantic.unresolved_name 'ISQ::MassValue'
semantic.unresolved_name 'Real'
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
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
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
semantic.unresolved_name 'ISQ::power'
semantic.unresolved_name 'ISQ::mass'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'result'
semantic.unresolved_name 'Real'
semantic.unresolved_name 'selectedAlternative'
~~~
# SMG
~~~
(semantic-graph
  (containment
    (element (kind "package") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations"))) (name "10b-Trade-off Among Alternative Configurations") (declared-name "10b-Trade-off Among Alternative Configurations")
      (contains
        (element (kind "import") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import"))) (name "*") (declared-name "*"))
        (element (kind "import") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::*#import2"))) (name "*") (declared-name "*"))
        (element (kind "package") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis"))) (name "Analysis") (declared-name "Analysis")
          (contains
            (element (kind "calc def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (name "EngineEvaluation") (declared-name "EngineEvaluation")
              (contains
                (element (kind "documentation") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::_documentation"))) (name "") (effective (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::cost"))) (name "cost") (declared-name "cost") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::efficiency"))) (name "efficiency") (declared-name "efficiency") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
                (element (kind "return parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::evaluation"))) (name "evaluation") (declared-name "evaluation") (effective (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::mass"))) (name "mass") (declared-name "mass") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
                (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::power"))) (name "power") (declared-name "power") (declared (properties (direction "in"))) (effective (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation")))))
              )
            )
            (element (kind "analysis") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (name "engineTradeStudy") (declared-name "engineTradeStudy")
              (contains
                (element (kind "subject") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (name ""))
                (element (kind "calc") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (name "evaluationFunction") (declared-name "evaluationFunction")
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (name "anEngine") (declared-name "anEngine") (declared (properties (direction "in") (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false))))
                    (element (kind "calc") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (name "costRollup") (declared-name "costRollup")
                      (contains
                        (element (kind "return parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::cost"))) (name "cost") (declared-name "cost"))
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup::engine"))) (name "engine") (declared-name "engine") (declared (properties (direction "in")) (own-expression (expression (kind "featureReference") (reference "anEngine")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                      )
                    )
                    (element (kind "calc") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (name "efficiencyRollup") (declared-name "efficiencyRollup")
                      (contains
                        (element (kind "return parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::efficiency"))) (name "efficiency") (declared-name "efficiency"))
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup::engine"))) (name "engine") (declared-name "engine") (declared (properties (direction "in")) (own-expression (expression (kind "featureReference") (reference "anEngine")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                      )
                    )
                    (element (kind "calc") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (name "massRollup") (declared-name "massRollup")
                      (contains
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::engine"))) (name "engine") (declared-name "engine") (declared (properties (direction "in")) (own-expression (expression (kind "featureReference") (reference "anEngine")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                        (element (kind "return parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup::mass"))) (name "mass") (declared-name "mass"))
                      )
                    )
                    (element (kind "calc") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (name "powerRollup") (declared-name "powerRollup")
                      (contains
                        (element (kind "in out parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::engine"))) (name "engine") (declared-name "engine") (declared (properties (direction "in")) (own-expression (expression (kind "featureReference") (reference "anEngine")))) (evaluation (expression (status "incomplete") (error "expression is incomplete"))))
                        (element (kind "return parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup::power"))) (name "power") (declared-name "power"))
                      )
                    )
                    (element (kind "return parameter") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::result"))) (name "result") (declared-name "result"))
                  )
                )
                (element (kind "objective") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::objective"))) (name "objective") (declared-name "objective"))
                (element (kind "analysis result") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (name "selectedAlternative") (declared-name "selectedAlternative"))
              )
            )
          )
        )
        (element (kind "package") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions"))) (name "Definitions") (declared-name "Definitions")
          (contains
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (name "4CylCrankShaft") (declared-name "4CylCrankShaft") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (name "6CylCrankShaft") (declared-name "6CylCrankShaft") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (name "ConnectingRod") (declared-name "ConnectingRod") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (name "CrankShaft") (declared-name "CrankShaft") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (name "Cylinder") (declared-name "Cylinder") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (name "Engine") (declared-name "Engine") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (name "Piston") (declared-name "Piston") (declared))
            (element (kind "part def") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (name "Vehicle") (declared-name "Vehicle") (declared))
          )
        )
        (element (kind "import") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Real"))) (name "Real") (declared-name "Real"))
        (element (kind "package") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages"))) (name "Usages") (declared-name "Usages")
          (contains
            (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (name "cs") (declared-name "cs") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine")))))
                (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (name "cyl") (declared-name "cyl") (declared (properties (ordered false)) (multiplicity (lower unbounded) (upper unbounded) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (name "p") (declared-name "p") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder")))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (name "rod") (declared-name "rod") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder")))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (name "engineChoice") (declared-name "engineChoice") (declared (properties (variation true) (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (name "4cylEngine") (declared-name "4cylEngine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (name "cs") (declared-name "cs") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 4) (upper 4) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
                (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (name "6cylEngine") (declared-name "6cylEngine") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false)))
                  (contains
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (name "cs") (declared-name "cs") (declared (properties (ordered false))) (effective (implied-multiplicity (lower 1) (upper 1) (ordered false)) (implied-feature-ownership (composite true) (reference false))))
                    (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (name "cyl") (declared (properties (ordered false)) (multiplicity (lower 6) (upper 6) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false))))
                  )
                )
              )
            )
            (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (name "vehicle") (declared-name "vehicle") (declared (properties (ordered false)))
              (contains
                (element (kind "part") (id (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (name "engine") (declared-name "engine") (declared (properties (ordered false)) (multiplicity (lower 1) (upper 1) (ordered false) (provenance authored))) (effective (implied-feature-ownership (composite true) (reference false)) (featuring-type (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle")))))
              )
            )
          )
        )
      )
    )
  )
  (relationships
    (annotation (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation::_documentation"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored))
    (specializes (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored))
    (subject (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored))
    (subsetting (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::selectedAlternative"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (provenance authored))
    (typing (status resolved) (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (to (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (provenance authored))
  )
  (pending-relationships
  )
  (pending-expression-relationships
  )
  (derived-relationship-resolutions
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::EngineEvaluation"))) (status missing-prerequisite) (target "Calculations::Calculation"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy"))) (status missing-prerequisite) (target "AnalysisCases::analysisCases"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::anEngine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction"))) (status missing-prerequisite) (target "Calculations::calculations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::costRollup"))) (status missing-prerequisite) (target "Calculations::calculations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::efficiencyRollup"))) (status missing-prerequisite) (target "Calculations::calculations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::massRollup"))) (status missing-prerequisite) (target "Calculations::calculations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::evaluationFunction::powerRollup"))) (status missing-prerequisite) (target "Calculations::calculations"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Analysis::engineTradeStudy::objective"))) (status missing-prerequisite) (target "Requirements::requirementChecks"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Cylinder"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Engine"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Piston"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Definitions::Vehicle"))) (status missing-prerequisite) (target "Parts::Part"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cs"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::p"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engine::cyl::rod"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cs"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::4cylEngine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cs"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::engineChoice::6cylEngine::cyl"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle"))) (status missing-prerequisite) (target "Parts::parts"))
    (universal-standard-library-relationship (from (node (document "d0") (qualified-name "10b-Trade-off Among Alternative Configurations::Usages::vehicle::engine"))) (status missing-prerequisite) (target "Parts::parts"))
  )
)
~~~
# DIAGNOSTICS
~~~sexpr
(fixture-diagnostics
  (document "sysml/validation/10b_trade_off_among_alternative_configurations.md"
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 41 4) (end 41 20))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 46 4) (end 46 20))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 66 3) (end 66 30))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 67 3) (end 67 28))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 68 3) (end 68 24))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 69 3) (end 69 18))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 70 3) (end 70 28))
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
        (code "unresolved_redefines_target")
        (source "semantic")
        (range (start 79 4) (end 79 46))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 81 45) (end 81 70))
      )
      (diagnostic
        (severity warning)
        (code "invalid_qualified_name_segment")
        (source "semantic")
        (range (start 82 44) (end 82 67))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 83 50) (end 83 74))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 84 44) (end 84 62))
      )
      (diagnostic
        (severity warning)
        (code "unresolved_type_reference")
        (source "semantic")
        (range (start 86 4) (end 86 141))
      )
    )
  )
)
~~~
