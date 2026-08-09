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

    calc def PowerRollup {
        in engine : Engine;
        return : ISQ::PowerValue;
    }
    calc def MassRollup {
        in engine : Engine;
        return : ISQ::MassValue;
    }
    calc def EfficiencyRollup {
        in engine : Engine;
        return : Real;
    }
    calc def CostRollup {
        in engine : Engine;
        return : Real;
    }

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

            calc powerRollup : PowerRollup {
                in engine = anEngine;
                return power;
            }
            calc massRollup : MassRollup {
                in engine = anEngine;
                return mass;
            }
            calc efficiencyRollup : EfficiencyRollup {
                in engine = anEngine;
                return efficiency;
            }
            calc costRollup : CostRollup {
                in engine = anEngine;
                return cost;
            }

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
(model
  (namespace
    (package 'Trade Study Analysis Example'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'TradeStudies'[unresolved])
      (part_def 'Engine')
      (part_usage 'engine4cyl' : 'Trade Study Analysis Example::Engine'[part_def])
      (part_usage 'engine6cyl' : 'Trade Study Analysis Example::Engine'[part_def])
      (calculation_def 'PowerRollup'
        (reference_usage in reference 'engine' : 'Trade Study Analysis Example::Engine'[part_def])
        (return_parameter_membership
          (feature_def out : 'ISQ::PowerValue'[unresolved])))
      (calculation_def 'MassRollup'
        (reference_usage in reference 'engine' : 'Trade Study Analysis Example::Engine'[part_def])
        (return_parameter_membership
          (feature_def out : 'ISQ::MassValue'[unresolved])))
      (calculation_def 'EfficiencyRollup'
        (reference_usage in reference 'engine' : 'Trade Study Analysis Example::Engine'[part_def])
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved])))
      (calculation_def 'CostRollup'
        (reference_usage in reference 'engine' : 'Trade Study Analysis Example::Engine'[part_def])
        (return_parameter_membership
          (feature_def out : 'Real'[unresolved])))
      (calculation_def 'EngineEvaluation'
        (reference_usage in reference 'power' : 'ISQ::PowerValue'[unresolved])
        (reference_usage in reference 'mass' : 'ISQ::MassValue'[unresolved])
        (reference_usage in reference 'efficiency' : 'Real'[unresolved])
        (reference_usage in reference 'cost' : 'Real'[unresolved])
        (return_parameter_membership
          (feature_def out 'evaluation' : 'Real'[unresolved])))
      (analysis_case_usage 'engineTradeStudy' : 'TradeStudy'[unresolved]
        (subject_membership in : 'Trade Study Analysis Example::Engine'[part_def]
          (feature_value (=)))
        (objective_membership composite : 'MaximizeObjective'[unresolved])
        (calculation_usage composite :>> 'evaluationFunction'[unresolved]
          (part_usage in 'anEngine' :>> 'alternative'[unresolved] : 'Trade Study Analysis Example::Engine'[part_def])
          (calculation_usage composite 'powerRollup' : 'Trade Study Analysis Example::PowerRollup'[calculation_def]
            (reference_usage in reference 'engine'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'power')))
          (calculation_usage composite 'massRollup' : 'Trade Study Analysis Example::MassRollup'[calculation_def]
            (reference_usage in reference 'engine'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'mass')))
          (calculation_usage composite 'efficiencyRollup' : 'Trade Study Analysis Example::EfficiencyRollup'[calculation_def]
            (reference_usage in reference 'engine'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'efficiency')))
          (calculation_usage composite 'costRollup' : 'Trade Study Analysis Example::CostRollup'[calculation_def]
            (reference_usage in reference 'engine'
              (feature_value (=)))
            (return_parameter_membership
              (feature_def out 'cost')))
          (return_parameter_membership
            (feature_def out :>> 'result'[unresolved] : 'Real'[unresolved]
              (feature_value (=)))))
        (return_parameter_membership
          (part_usage out :>> 'selectedAlternative'[unresolved] : 'Trade Study Analysis Example::Engine'[part_def]))))))
~~~
