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
            part cyl : Cylinder [*] {
                part p : Piston [1];
                part rod : ConnectingRod [1];
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
            part engine :> engineChoice [1] = engineChoice::'6cylEngine' {
                assert constraint engineSelectionRational {
                    doc /* Selected the best engine based on the 'engineTradeStudy'. */
                    = engine == Analysis::engineTradeStudy.selectedAlternative;
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
            subject : Engine [1..*] = all engineChoice;
            objective : MaximizeObjective;

            calc :>> evaluationFunction {
                in part anEngine :>> alternative : Engine;

                calc powerRollup {
                    in engine = anEngine;
                    return power:>ISQ::power;
                }
                calc massRollup {
                    in engine = anEngine;
                    return mass:>ISQ::mass;
                }
                calc efficiencyRollup {
                    in engine = anEngine;
                    return efficiency: Real;
                }
                calc costRollup {
                    in engine = anEngine;
                    return cost: Real;
                }

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
(model
  (namespace
    (package '10b-Trade-off Among Alternative Configurations'
      (membership_import private -> 'ScalarValues::Real'[unresolved])
      (namespace_import private -> 'TradeStudies'[unresolved])
      (namespace_import private -> '10b-Trade-off Among Alternative Configurations::Definitions'[package])
      (namespace_import private -> '10b-Trade-off Among Alternative Configurations::Usages'[package])
      (package 'Definitions'
        (part_def 'Vehicle')
        (part_def 'Engine'
          (reference_usage reference 'power' : 'ISQ::PowerValue'[unresolved])
          (reference_usage reference 'mass' : 'ISQ::MassValue'[unresolved])
          (reference_usage reference 'efficiency' : 'Real'[unresolved])
          (reference_usage reference 'reliability' : 'Real'[unresolved])
          (reference_usage reference 'cost' : 'Real'[unresolved]))
        (part_def 'Piston')
        (part_def 'Cylinder')
        (part_def 'ConnectingRod')
        (part_def 'CrankShaft')
        (part_def '4CylCrankShaft' :> '10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft'[part_def])
        (part_def '6CylCrankShaft' :> '10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft'[part_def]))
      (package 'Usages'
        (part_usage 'engine' : '10b-Trade-off Among Alternative Configurations::Definitions::Engine'[part_def]
          (part_usage composite 'cyl' : '10b-Trade-off Among Alternative Configurations::Definitions::Cylinder'[part_def]
            (multiplicity_range [*])
            (part_usage composite 'p' : '10b-Trade-off Among Alternative Configurations::Definitions::Piston'[part_def]
              (multiplicity_range [1]))
            (part_usage composite 'rod' : '10b-Trade-off Among Alternative Configurations::Definitions::ConnectingRod'[part_def]
              (multiplicity_range [1])))
          (part_usage composite 'cs' : '10b-Trade-off Among Alternative Configurations::Definitions::CrankShaft'[part_def]))
        (part_usage variation 'engineChoice' :> '10b-Trade-off Among Alternative Configurations::Usages::engine'[part_usage]
          (variant_usage
            (part_usage composite '4cylEngine'
              (part_usage composite :>> '10b-Trade-off Among Alternative Configurations::Usages::engine::cyl'[part_usage]
                (multiplicity_range [4]))
              (part_usage composite :>> '10b-Trade-off Among Alternative Configurations::Usages::engine::cs'[part_usage] : '10b-Trade-off Among Alternative Configurations::Definitions::4CylCrankShaft'[part_def])))
          (variant_usage
            (part_usage composite '6cylEngine'
              (part_usage composite :>> '10b-Trade-off Among Alternative Configurations::Usages::engine::cyl'[part_usage]
                (multiplicity_range [6]))
              (part_usage composite :>> '10b-Trade-off Among Alternative Configurations::Usages::engine::cs'[part_usage] : '10b-Trade-off Among Alternative Configurations::Definitions::6CylCrankShaft'[part_def]))))
        (part_usage 'vehicle' : '10b-Trade-off Among Alternative Configurations::Definitions::Vehicle'[part_def]
          (part_usage composite 'engine' :> '10b-Trade-off Among Alternative Configurations::Usages::engineChoice'[part_usage]
            (multiplicity_range [1])
            (feature_value (=))
            (assert_constraint_usage 'engineSelectionRational'
              (documentation)
              (result_expr_membership)))))
      (package 'Analysis'
        (calculation_def 'EngineEvaluation'
          (documentation)
          (reference_usage in reference 'power' : 'ISQ::PowerValue'[unresolved])
          (reference_usage in reference 'mass' : 'ISQ::MassValue'[unresolved])
          (reference_usage in reference 'efficiency' : 'Real'[unresolved])
          (reference_usage in reference 'cost' : 'Real'[unresolved])
          (return_parameter_membership
            (feature_def out 'evaluation' : 'Real'[unresolved])))
        (analysis_case_usage 'engineTradeStudy' : 'TradeStudy'[unresolved]
          (subject_membership in : '10b-Trade-off Among Alternative Configurations::Definitions::Engine'[part_def]
            (multiplicity_range [1..*])
            (feature_value (=)))
          (objective_membership composite : 'MaximizeObjective'[unresolved])
          (calculation_usage composite :>> 'evaluationFunction'[unresolved]
            (part_usage in 'anEngine' :>> 'alternative'[unresolved] : '10b-Trade-off Among Alternative Configurations::Definitions::Engine'[part_def])
            (calculation_usage composite 'powerRollup'
              (reference_usage in reference 'engine'
                (feature_value (=)))
              (return_parameter_membership
                (feature_def out 'power' :> 'ISQ::power'[unresolved])))
            (calculation_usage composite 'massRollup'
              (reference_usage in reference 'engine'
                (feature_value (=)))
              (return_parameter_membership
                (feature_def out 'mass' :> 'ISQ::mass'[unresolved])))
            (calculation_usage composite 'efficiencyRollup'
              (reference_usage in reference 'engine'
                (feature_value (=)))
              (return_parameter_membership
                (feature_def out 'efficiency' : 'Real'[unresolved])))
            (calculation_usage composite 'costRollup'
              (reference_usage in reference 'engine'
                (feature_value (=)))
              (return_parameter_membership
                (feature_def out 'cost' : 'Real'[unresolved])))
            (return_parameter_membership
              (feature_def out :>> 'result'[unresolved] : 'Real'[unresolved]
                (feature_value (=)))))
          (return_parameter_membership
            (part_usage out :>> 'selectedAlternative'[unresolved] : '10b-Trade-off Among Alternative Configurations::Definitions::Engine'[part_def])))))))
~~~
