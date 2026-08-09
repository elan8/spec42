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
    part engine1 : Engine;
    part engine2 : Engine;

    analysis engineTradeStudy : TradeStudy {
        subject : Engine [1..*] = (engine1, engine2);
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
(model
  (namespace
    (package 'TradeStudyTest'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'TradeStudies'[unresolved])
      (part_def 'Engine')
      (part_usage 'engine1' : 'TradeStudyTest::Engine'[part_def])
      (part_usage 'engine2' : 'TradeStudyTest::Engine'[part_def])
      (analysis_case_usage 'engineTradeStudy' : 'TradeStudy'[unresolved]
        (subject_membership in : 'TradeStudyTest::Engine'[part_def]
          (multiplicity_range [1..*])
          (feature_value (=)))
        (objective_membership composite : 'MaximizeObjective'[unresolved])
        (calculation_usage composite :>> 'evaluationFunction'[unresolved]
          (part_usage in : 'TradeStudyTest::Engine'[part_def])
          (return_parameter_membership
            (feature_def out : 'Real'[unresolved])))
        (return_parameter_membership
          (part_usage out : 'TradeStudyTest::Engine'[part_def]))))))
~~~
